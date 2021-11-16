use anyhow::Context;
use tonic::transport::Channel;
use tonic::Streaming;

pub use crate::client::iotics::api::interest_api_client::InterestApiClient;
pub use crate::client::iotics::api::{FetchInterestRequest, FetchInterestResponse};

use crate::client::iotics::api::fetch_interest_request::Arguments;
use crate::client::iotics::api::interest::FollowedFeed;
use crate::client::iotics::api::Feed;
use crate::client::iotics::api::Interest;
use crate::client::iotics::api::{FeedId, Headers, HostId, TwinId};
use crate::helpers::generate_client_app_id;

pub async fn create_interest_api_client(
    host_address: &str,
) -> Result<InterestApiClient<Channel>, anyhow::Error> {
    let client = InterestApiClient::connect(host_address.to_string()).await?;

    Ok(client)
}

pub async fn follow(
    host_address: &str,
    token: &str,
    followed_host_id: Option<HostId>,
    followed_twin_id: TwinId,
    followed_feed: String,
    follower_twin_id: TwinId,
    fetch_last_stored: bool,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let mut client = create_interest_api_client(host_address).await?;

    follow_with_client(
        &mut client,
        token,
        followed_host_id,
        followed_twin_id,
        followed_feed,
        follower_twin_id,
        fetch_last_stored,
    )
    .await
}

pub async fn follow_with_client(
    client: &mut InterestApiClient<Channel>,
    token: &str,
    followed_host_id: Option<HostId>,
    followed_twin_id: TwinId,
    followed_feed: String,
    follower_twin_id: TwinId,
    fetch_last_stored: bool,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let client_app_id = generate_client_app_id();

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: vec![client_app_id],
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

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let stream = client.fetch_interests(request).await?.into_inner();

    Ok(stream)
}
