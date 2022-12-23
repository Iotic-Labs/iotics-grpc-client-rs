use anyhow::Context;
use std::sync::Arc;
use tonic::transport::Channel;

use crate::client::iotics::api::describe_feed_request::Arguments as DescribeFeedRequestArguments;
use crate::client::iotics::api::describe_twin_request::Arguments as DescribeTwinRequestArguments;
use crate::client::iotics::api::feed_api_client::FeedApiClient;
use crate::client::iotics::api::twin_api_client::TwinApiClient;
use crate::client::iotics::api::{
    DescribeFeedRequest, DescribeTwinRequest, FeedId, Headers, TwinId,
};

use crate::auth_builder::IntoAuthBuilder;
use crate::channel::create_channel;
use crate::helpers::generate_client_app_id;
use crate::twin::{DescribeFeedResponse, DescribeTwinResponse};

pub async fn describe_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    remote_host_id: Option<&str>,
) -> Result<DescribeTwinResponse, anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    describe_twin_with_channel(auth_builder, channel, twin_id, remote_host_id).await
}

pub async fn describe_twin_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
    remote_host_id: Option<&str>,
) -> Result<DescribeTwinResponse, anyhow::Error> {
    let mut client = TwinApiClient::new(channel);
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let twin_id_arg = TwinId {
        id: twin_id.to_string(),
        host_id: remote_host_id.unwrap_or_default().to_string(),
    };

    let args = DescribeTwinRequestArguments {
        twin_id: Some(twin_id_arg),
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

    let result = client.describe_twin(request).await.with_context(|| {
        format!(
            "Describing twin failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;
    let result = result.into_inner();

    Ok(result)
}

pub async fn describe_feed(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    feed_id: &str,
    remote_host_id: Option<&str>,
) -> Result<DescribeFeedResponse, anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    describe_feed_with_channel(auth_builder, channel, twin_id, feed_id, remote_host_id).await
}

pub async fn describe_feed_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
    feed_id: &str,
    remote_host_id: Option<&str>,
) -> Result<DescribeFeedResponse, anyhow::Error> {
    let mut client = FeedApiClient::new(channel);
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let args = DescribeFeedRequestArguments {
        feed_id: Some(FeedId {
            id: feed_id.to_string(),
            twin_id: twin_id.to_string(),
            host_id: remote_host_id.unwrap_or_default().to_string(),
        }),
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

    let result = client.describe_feed(request).await.with_context(|| {
        format!(
            "Describing feed failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;
    let result = result.into_inner();

    Ok(result)
}
