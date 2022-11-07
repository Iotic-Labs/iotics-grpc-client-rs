use anyhow::Context;
use std::result::Result;
use std::sync::Arc;
use tonic::transport::Channel;

pub use crate::client::iotics::api::host_api_client::HostApiClient;
pub use crate::client::iotics::api::{GetHostIdRequest, GetHostIdResponse, Headers};

use crate::auth_builder::IntoAuthBuilder;
use crate::helpers::generate_client_app_id;

pub async fn create_host_api_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<HostApiClient<Channel>, anyhow::Error> {
    let host_address = auth_builder.get_host()?;
    let client = HostApiClient::connect(host_address).await?;

    Ok(client)
}

pub async fn get_local_host_id(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<GetHostIdResponse, anyhow::Error> {
    let mut client: HostApiClient<Channel> = create_host_api_client(auth_builder.clone()).await?;

    get_local_host_id_client(auth_builder, &mut client).await
}

pub async fn get_local_host_id_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut HostApiClient<Channel>,
) -> Result<GetHostIdResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let mut request = tonic::Request::new(GetHostIdRequest {
        headers: Some(headers),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.get_host_id(request).await.with_context(|| {
        format!(
            "Getting local host id failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;
    let result = result.into_inner();

    Ok(result)
}
