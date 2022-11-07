use anyhow::Context;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tonic::transport::{Channel, Endpoint};
use tonic::Streaming;

use crate::client::google::protobuf::{BoolValue, Timestamp};
use crate::client::iotics::api::fetch_interest_request::Arguments as FetchInterestArguments;
use crate::client::iotics::api::{InputInterest, InputMessage, Interest};

pub use crate::client::iotics::api::interest_api_client::InterestApiClient;
pub use crate::client::iotics::api::send_input_message_request::{
    Arguments as SendMessageArguments, Payload,
};

pub use crate::client::iotics::api::{
    FeedId, FetchInterestRequest, FetchInterestResponse, Headers, InputId, SendInputMessageRequest,
    SendInputMessageResponse, TwinId,
};

use crate::auth_builder::IntoAuthBuilder;
use crate::helpers::generate_client_app_id;

pub async fn create_interest_api_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    keep_alive_interval: Option<Duration>,
) -> Result<InterestApiClient<Channel>, anyhow::Error> {
    let host_address = auth_builder.get_host()?;

    let conn = match keep_alive_interval {
        Some(ka) => Endpoint::new(host_address)?.http2_keep_alive_interval(ka),
        None => Endpoint::new(host_address)?,
    };

    let client = InterestApiClient::new(conn.connect().await?);

    Ok(client)
}

pub async fn follow(
    auth_builder: Arc<impl IntoAuthBuilder>,
    followed_host_id: Option<&str>,
    followed_twin_id: &str,
    followed_feed_id: &str,
    follower_twin_id: &str,
    fetch_last_stored: bool,
    keep_alive_interval: Option<Duration>,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let mut client = create_interest_api_client(auth_builder.clone(), keep_alive_interval).await?;

    follow_with_client(
        auth_builder,
        &mut client,
        followed_host_id,
        followed_twin_id,
        followed_feed_id,
        follower_twin_id,
        fetch_last_stored,
    )
    .await
}

pub async fn follow_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut InterestApiClient<Channel>,
    followed_host_id: Option<&str>,
    followed_twin_id: &str,
    followed_feed_id: &str,
    follower_twin_id: &str,
    fetch_last_stored: bool,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let follower_twin_id_arg = TwinId {
        id: follower_twin_id.to_string(),
        ..Default::default()
    };

    let interest = Interest {
        follower_twin_id: Some(follower_twin_id_arg),
        followed_feed_id: Some(FeedId {
            id: followed_feed_id.to_string(),
            twin_id: followed_twin_id.to_string(),
            host_id: followed_host_id.unwrap_or_default().to_string(),
        }),
    };
    let mut request = tonic::Request::new(FetchInterestRequest {
        headers: Some(headers),
        args: Some(FetchInterestArguments {
            interest: Some(interest),
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
    let mut client = create_interest_api_client(auth_builder.clone(), None).await?;

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

    let sender_twin_id = TwinId {
        id: sender_twin_id.to_string(),
        ..Default::default()
    };

    let dest_input_id = InputId {
        id: input_id.to_string(),
        twin_id: receiver_twin_id.to_string(),
        host_id: receiver_host_id.unwrap_or_default().to_string(),
    };

    let interest = InputInterest {
        dest_input_id: Some(dest_input_id),
        sender_twin_id: Some(sender_twin_id),
    };

    let args = SendMessageArguments {
        interest: Some(interest),
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
