use anyhow::Context;
use std::sync::Arc;
use tonic::Request;

use crate::client::iotics::api::twin_api_client::TwinApiClient;
use crate::client::iotics::api::upsert_twin_request::Payload as UpsertTwinRequestPayload;
use crate::client::iotics::api::UpsertTwinRequest;

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, GeoLocation, Headers, Property, Response};
use crate::helpers::generate_client_app_id;

use super::{create_twin_api_client, UpsertFeedWithMeta, UpsertTwinResponse};

#[allow(clippy::too_many_arguments)]
pub async fn upsert_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    did: &str,
    properties: Vec<Property>,
    feeds: Vec<UpsertFeedWithMeta>,
    location: Option<GeoLocation>,
    visibility: i32,
) -> Result<Response<UpsertTwinResponse>, anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    upsert_twin_with_client(
        auth_builder,
        &mut client,
        did,
        properties,
        feeds,
        location,
        visibility,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_twin_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
    did: &str,
    properties: Vec<Property>,
    feeds: Vec<UpsertFeedWithMeta>,
    location: Option<GeoLocation>,
    visibility: i32,
) -> Result<Response<UpsertTwinResponse>, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let payload = UpsertTwinRequestPayload {
        twin_id: did.to_string(),
        properties,
        feeds,
        location,
        visibility,
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
