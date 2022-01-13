use anyhow::Context;
use std::sync::Arc;

use crate::client::iotics::api::describe_feed_request::Arguments as DescribeFeedRequestArguments;
use crate::client::iotics::api::describe_twin_request::Arguments as DescribeTwinRequestArguments;
use crate::client::iotics::api::{DescribeFeedRequest, DescribeTwinRequest, Feed, HostId};

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, FeedId, Headers, TwinId};
use crate::helpers::generate_client_app_id;

use super::{
    create_feed_api_client, create_twin_api_client, DescribeFeedResponse, DescribeTwinResponse,
    FeedApiClient, TwinApiClient,
};

pub async fn describe_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: TwinId,
    remote_host_id: Option<HostId>,
) -> Result<DescribeTwinResponse, anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    describe_twin_with_client(auth_builder, &mut client, twin_id, remote_host_id).await
}

pub async fn describe_twin_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
    twin_id: TwinId,
    remote_host_id: Option<HostId>,
) -> Result<DescribeTwinResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref,
        ..Default::default()
    };

    let args = DescribeTwinRequestArguments {
        twin_id: Some(twin_id),
        remote_host_id,
    };

    let mut request = tonic::Request::new(DescribeTwinRequest {
        headers: Some(headers),
        args: Some(args),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.describe_twin(request).await?;

    let result = result.into_inner();

    Ok(result)
}

pub async fn describe_feed(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: TwinId,
    feed_id: FeedId,
    remote_host_id: Option<HostId>,
) -> Result<DescribeFeedResponse, anyhow::Error> {
    let mut client = create_feed_api_client(auth_builder.clone()).await?;

    describe_feed_with_client(auth_builder, &mut client, twin_id, feed_id, remote_host_id).await
}

pub async fn describe_feed_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut FeedApiClient<Channel>,
    twin_id: TwinId,
    feed_id: FeedId,
    remote_host_id: Option<HostId>,
) -> Result<DescribeFeedResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref,
        ..Default::default()
    };

    let args = DescribeFeedRequestArguments {
        feed: Some(Feed {
            id: Some(feed_id),
            twin_id: Some(twin_id),
        }),
        remote_host_id,
    };

    let mut request = tonic::Request::new(DescribeFeedRequest {
        headers: Some(headers),
        args: Some(args),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.describe_feed(request).await?;

    let result = result.into_inner();

    Ok(result)
}
