pub mod crud;
pub mod describe;
pub mod list;
pub mod share;
pub mod upsert;

use std::sync::Arc;

pub use crate::client::iotics::api::feed_api_client::FeedApiClient;
pub use crate::client::iotics::api::list_all_twins_response::TwinDetails;
pub use crate::client::iotics::api::twin_api_client::TwinApiClient;
pub use crate::client::iotics::api::{
    DescribeFeedResponse, DescribeTwinResponse, FetchInterestRequest, FetchInterestResponse,
    ListAllTwinsResponse, UpsertFeedWithMeta, UpsertInputWithMeta, UpsertTwinResponse,
};

use crate::auth_builder::IntoAuthBuilder;
use crate::common::Channel;

pub const PAGE_SIZE: u32 = 100;

pub async fn create_twin_api_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<TwinApiClient<Channel>, anyhow::Error> {
    let host_address = auth_builder.get_host()?;
    let client = TwinApiClient::connect(host_address).await?;

    Ok(client)
}

pub async fn create_feed_api_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<FeedApiClient<Channel>, anyhow::Error> {
    let host_address = auth_builder.get_host()?;
    let client = FeedApiClient::connect(host_address).await?;

    Ok(client)
}
