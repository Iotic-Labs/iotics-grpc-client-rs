use anyhow::Context;
use std::sync::Arc;

use crate::client::iotics::api::ListAllTwinsRequest;

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, Headers};
use crate::helpers::generate_client_app_id;

use super::{create_twin_api_client, ListAllTwinsResponse, TwinApiClient};

pub async fn list_all_twins(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<ListAllTwinsResponse, anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    list_all_twins_with_client(auth_builder, &mut client).await
}

pub async fn list_all_twins_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
) -> Result<ListAllTwinsResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref,
        ..Default::default()
    };

    let mut request = tonic::Request::new(ListAllTwinsRequest {
        headers: Some(headers),
        range: None,
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.list_all_twins(request).await?;

    let result = result.into_inner();

    Ok(result)
}
