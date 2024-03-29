use anyhow::Context;
use std::sync::Arc;
use tonic::transport::Channel;
use tonic::{Request, Response};

use crate::client::iotics::api::twin_api_client::TwinApiClient;
use crate::client::iotics::api::upsert_twin_request::Payload as UpsertTwinRequestPayload;
use crate::client::iotics::api::{GeoLocation, Headers, Property, TwinId, UpsertTwinRequest};

use crate::auth_builder::IntoAuthBuilder;
use crate::channel::create_channel;
use crate::helpers::generate_client_app_id;
use crate::twin::{UpsertFeedWithMeta, UpsertInputWithMeta, UpsertTwinResponse};

#[allow(clippy::too_many_arguments)]
pub async fn upsert_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    properties: Vec<Property>,
    feeds: Vec<UpsertFeedWithMeta>,
    inputs: Vec<UpsertInputWithMeta>,
    location: Option<GeoLocation>,
) -> Result<Response<UpsertTwinResponse>, anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    upsert_twin_with_channel(
        auth_builder,
        channel,
        twin_id,
        properties,
        feeds,
        inputs,
        location,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_twin_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
    properties: Vec<Property>,
    feeds: Vec<UpsertFeedWithMeta>,
    inputs: Vec<UpsertInputWithMeta>,
    location: Option<GeoLocation>,
) -> Result<Response<UpsertTwinResponse>, anyhow::Error> {
    let mut client = TwinApiClient::new(channel);
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let payload = UpsertTwinRequestPayload {
        twin_id: Some(TwinId {
            id: twin_id.to_string(),
            ..Default::default()
        }),
        properties,
        feeds,
        inputs,
        location,
    };

    let mut request = Request::new(UpsertTwinRequest {
        headers: Some(headers),
        payload: Some(payload),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let response = client.upsert_twin(request).await.with_context(|| {
        format!(
            "Upserting twin failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;

    Ok(response)
}
