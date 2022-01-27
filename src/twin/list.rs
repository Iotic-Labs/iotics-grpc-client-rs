use anyhow::Context;
use std::sync::Arc;

use crate::client::iotics::api::ListAllTwinsRequest;

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, Headers, Limit, Offset, Range};
use crate::helpers::generate_client_app_id;

use super::{create_twin_api_client, ListAllTwinsResponse, TwinApiClient};

pub async fn list_all_twins(
    auth_builder: Arc<impl IntoAuthBuilder>,
    limit: u32,
    offset: u32,
) -> Result<ListAllTwinsResponse, anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    list_all_twins_with_client(auth_builder, &mut client, limit, offset).await
}

pub async fn list_all_twins_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
    limit: u32,
    offset: u32,
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
        range: Some(Range {
            limit: Some(Limit { value: limit }),
            offset: Some(Offset { value: offset }),
        }),
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
