use anyhow::Context;
use tonic::transport::Channel;
use tonic::Streaming;

pub use crate::api::interest::interest_api_client::InterestApiClient;
pub use crate::api::interest::{FetchInterestRequest, FetchInterestResponse};

use crate::api::common::{FeedId, Headers, HostId, TwinId};
use crate::api::feed::Feed;
use crate::api::interest::fetch_interest_request::Arguments;
use crate::api::interest::interest::FollowedFeed;
use crate::api::interest::Interest;
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
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let mut client = create_interest_api_client(host_address).await?;

    follow_with_client(
        &mut client,
        token,
        followed_host_id,
        followed_twin_id,
        followed_feed,
        follower_twin_id,
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
        ..Default::default()
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let stream = client.fetch_interests(request).await?.into_inner();

    Ok(stream)
}
