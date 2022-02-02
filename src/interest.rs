use anyhow::Context;
use std::sync::Arc;

use crate::client::iotics::api::fetch_interest_request::Arguments;
use crate::client::iotics::api::interest::FollowedFeed;
use crate::client::iotics::api::Feed;
use crate::client::iotics::api::Interest;

pub use crate::client::iotics::api::interest_api_client::InterestApiClient;
pub use crate::client::iotics::api::{FetchInterestRequest, FetchInterestResponse};

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, FeedId, Headers, HostId, Streaming, TwinId};
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
        args: Some(Arguments {
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
        fetch_last_stored: Some(fetch_last_stored),
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
