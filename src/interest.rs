use anyhow::Context;
use std::sync::Arc;
use std::time::SystemTime;

use crate::client::google::protobuf::{BoolValue, Timestamp};
use crate::client::iotics::api::fetch_interest_request::Arguments as FetchInterestArguments;
use crate::client::iotics::api::input_interest::DestinationInput;
use crate::client::iotics::api::interest::FollowedFeed;
use crate::client::iotics::api::{Feed, Input, InputInterest, InputMessage, Interest};

pub use crate::client::iotics::api::interest_api_client::InterestApiClient;
pub use crate::client::iotics::api::send_input_message_request::{
    Arguments as SendMessageArguments, Payload,
};

pub use crate::client::iotics::api::{
    FetchInterestRequest, FetchInterestResponse, SendInputMessageRequest, SendInputMessageResponse,
};

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, FeedId, Headers, HostId, InputId, Streaming, TwinId};
use crate::helpers::generate_client_app_id;

pub async fn create_interest_api_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<InterestApiClient<Channel>, anyhow::Error> {
    let host_address = auth_builder.get_host()?;
    let client = InterestApiClient::connect(host_address).await?;

    Ok(client)
}

pub async fn follow(
    auth_builder: Arc<impl IntoAuthBuilder>,
    followed_host_id: Option<HostId>,
    followed_twin_id: TwinId,
    followed_feed: String,
    follower_twin_id: TwinId,
    fetch_last_stored: bool,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let mut client = create_interest_api_client(auth_builder.clone()).await?;

    follow_with_client(
        auth_builder,
        &mut client,
        followed_host_id,
        followed_twin_id,
        followed_feed,
        follower_twin_id,
        fetch_last_stored,
    )
    .await
}

pub async fn follow_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut InterestApiClient<Channel>,
    followed_host_id: Option<HostId>,
    followed_twin_id: TwinId,
    followed_feed: String,
    follower_twin_id: TwinId,
    fetch_last_stored: bool,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let mut request = tonic::Request::new(FetchInterestRequest {
        headers: Some(headers),
        args: Some(FetchInterestArguments {
            interest: Some(Interest {
                followed_feed: Some(FollowedFeed {
                    feed: Some(Feed {
                        id: Some(FeedId {
                            value: followed_feed,
                        }),
                        twin_id: Some(followed_twin_id),
                    }),
                    host_id: followed_host_id,
                }),
                follower_twin_id: Some(follower_twin_id),
            }),
        }),
        fetch_last_stored: Some(BoolValue {
            value: fetch_last_stored,
        }),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let stream = client
        .fetch_interests(request)
        .await
        .with_context(|| {
            format!(
                "Fetching interests failed, transaction ref [{}]",
                transaction_ref.join(", ")
            )
        })?
        .into_inner();

    Ok(stream)
}

pub async fn send_input_message<T: Into<Vec<u8>>>(
    auth_builder: Arc<impl IntoAuthBuilder>,
    receiver_host_id: Option<&str>,
    receiver_twin_id: &str,
    input_id: &str,
    sender_twin_id: &str,
    data: T,
) -> Result<(), anyhow::Error> {
    let mut client = create_interest_api_client(auth_builder.clone()).await?;

    send_input_message_with_client(
        auth_builder,
        &mut client,
        receiver_host_id,
        receiver_twin_id,
        input_id,
        sender_twin_id,
        data,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn send_input_message_with_client<T: Into<Vec<u8>>>(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut InterestApiClient<Channel>,
    receiver_host_id: Option<&str>,
    receiver_twin_id: &str,
    input_id: &str,
    sender_twin_id: &str,
    data: T,
) -> Result<(), anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };
    let host_id = receiver_host_id.map(|id| HostId {
        value: id.to_string(),
    });

    let args = SendMessageArguments {
        interest: Some(InputInterest {
            dest_input: Some(DestinationInput {
                host_id,
                input: Some(Input {
                    id: Some(InputId {
                        value: input_id.to_string(),
                    }),
                    twin_id: Some(TwinId {
                        value: receiver_twin_id.to_string(),
                    }),
                }),
            }),
            sender_twin_id: Some(TwinId {
                value: sender_twin_id.to_string(),
            }),
        }),
    };

    let dtm = std::time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let timestamp = Timestamp {
        seconds: dtm.as_secs() as i64,
        nanos: 0,
    };

    let payload = Payload {
        message: Some(InputMessage {
            occurred_at: Some(timestamp),
            mime: "application/json".to_string(),
            data: data.into(),
        }),
    };

    let mut request = tonic::Request::new(SendInputMessageRequest {
        headers: Some(headers.clone()),
        args: Some(args.clone()),
        payload: Some(payload.clone()),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );
    if let Err(e) = client.send_input_message(request).await {
        return Err(anyhow::Error::new(e).context(format!(
            "Sending message failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )));
    }

    Ok(())
}
