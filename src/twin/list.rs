use anyhow::Context;
use std::sync::Arc;

use crate::client::iotics::api::ListAllTwinsRequest;

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, Headers, Limit, Offset, Range};
use crate::helpers::generate_client_app_id;

use super::{create_twin_api_client, Twin, TwinApiClient, PAGE_SIZE};

pub async fn list_all_twins(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<Vec<Twin>, anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    list_all_twins_with_client(auth_builder, &mut client).await
}

pub async fn list_all_twins_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
) -> Result<Vec<Twin>, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let mut twins = Vec::new();
    let mut current_page = 0;
    let mut get_next_page = true;

    while get_next_page {
        let mut request = tonic::Request::new(ListAllTwinsRequest {
            headers: Some(headers.clone()),
            range: Some(Range {
                limit: Some(Limit { value: PAGE_SIZE }),
                offset: Some(Offset {
                    value: PAGE_SIZE * current_page,
                }),
            }),
        });

        let token = auth_builder.get_token()?;

        request.metadata_mut().append(
            "authorization",
            token.parse().context("parse token failed")?,
        );

        let result = client.list_all_twins(request).await.with_context(|| {
            format!(
                "Listing twins failed, transaction ref [{}]",
                transaction_ref.join(", ")
            )
        })?;
        let result = result.into_inner();

        let payload = result.payload.context("failed to find payload")?;

        if payload.twins.len() < PAGE_SIZE as usize {
            get_next_page = false;
        }

        twins.extend(payload.twins);
        current_page += 1;
    }

    Ok(twins)
}
