use anyhow::Context;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver};
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Channel;

pub use crate::client::iotics::api::search_api_client::SearchApiClient;
pub use crate::client::iotics::api::search_request::payload::Filter;
pub use crate::client::iotics::api::search_response::{
    FeedDetails, Payload as SearchResponsePayload, TwinDetails,
};
pub use crate::client::iotics::api::{ResponseType, SearchRequest, SearchResponse};
pub const SEARCH_PAGE_SIZE: u32 = 100;

use crate::client::iotics::api::search_request::Payload as SearchRequestPayload;
use crate::client::iotics::api::{Headers, Limit, Offset, Range, Scope, SubscriptionHeaders};
use crate::helpers::generate_client_app_id;

pub async fn create_search_api_client(
    host_address: &str,
) -> Result<SearchApiClient<Channel>, anyhow::Error> {
    let client = SearchApiClient::connect(host_address.to_string()).await?;

    Ok(client)
}

pub async fn search(
    host_address: &str,
    token: &str,
    filter: Filter,
    scope: Scope,
    timeout: Option<Duration>,
) -> Result<Receiver<Result<SearchResponse, anyhow::Error>>, anyhow::Error> {
    let client = create_search_api_client(host_address).await?;

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let (tx, rx) = channel::<Result<SearchResponse, anyhow::Error>>(16384);

    let results_client = client.clone();
    let results_token = token.to_string();
    let results_client_app_id = client_app_id.clone();
    let results_transaction_ref = transaction_ref.clone();
    let results_filter = filter.clone();

    let page = Arc::new(AtomicU32::new(0));

    let fut = async move {
        let mut request = tonic::Request::new(SubscriptionHeaders {
            client_app_id: results_client_app_id.clone(),
            transaction_ref: results_transaction_ref.clone(),
            ..Default::default()
        });

        let token: Result<MetadataValue<Ascii>, _> =
            results_token.clone().parse().context("parse token failed");

        match token {
            Ok(token) => {
                request.metadata_mut().append("authorization", token);

                let stream = results_client
                    .clone()
                    .receive_all_search_responses(request)
                    .await;

                match stream {
                    Ok(mut stream) => {
                        let stream = stream.get_mut();

                        while let Ok(Some(result)) = stream.message().await {
                            if let Some(payload) = &result.payload {
                                if payload.twins.len() >= SEARCH_PAGE_SIZE as usize {
                                    let current_page = page.load(Ordering::SeqCst);

                                    if result.headers.as_ref().unwrap().client_ref
                                        == format!("{}_{}", &results_client_app_id, current_page)
                                    {
                                        let response = search_page(
                                            results_client.clone(),
                                            &results_token,
                                            results_filter.clone(),
                                            scope,
                                            current_page + 1,
                                            results_client_app_id.clone(),
                                            results_transaction_ref.clone(),
                                        )
                                        .await;

                                        match response {
                                            Ok(_) => {
                                                page.fetch_add(1, Ordering::SeqCst);
                                            }
                                            Err(e) => {
                                                // ignore the potential error, the stream must be closed
                                                let _ = tx.send(Err(e)).await;
                                            }
                                        }
                                    }
                                }

                                // ignore the potential error, the stream must be closed
                                let _ = tx.send(Ok(result)).await;
                            }
                        }
                    }
                    Err(e) => {
                        // ignore the potential error, the stream must be closed
                        let _ = tx.send(Err(e.into())).await;
                    }
                }
            }
            Err(e) => {
                // ignore the potential error, the stream must be closed
                let _ = tx.send(Err(e)).await;
            }
        }
    };

    let handle = tokio::spawn(fut);

    if let Some(timeout) = timeout {
        tokio::spawn(async move {
            tokio::time::sleep(timeout).await;
            handle.abort();
        });
    }

    search_page(
        client,
        token,
        filter,
        scope,
        0,
        client_app_id,
        transaction_ref,
    )
    .await?;

    Ok(rx)
}

async fn search_page(
    mut client: SearchApiClient<Channel>,
    token: &str,
    filter: Filter,
    scope: Scope,
    page: u32,
    client_app_id: String,
    transaction_ref: Vec<String>,
) -> Result<(), anyhow::Error> {
    let headers = Headers {
        client_app_id: client_app_id.clone(),
        client_ref: format!("{}_{}", client_app_id, page),
        transaction_ref,
        ..Default::default()
    };

    let payload = SearchRequestPayload {
        filter: Some(filter),
        response_type: ResponseType::Full as i32,
        ..Default::default()
    };

    let mut request = tonic::Request::new(SearchRequest {
        lang: Some("en".to_string()),
        scope: scope as i32,
        payload: Some(payload),
        headers: Some(headers),
        range: Some(Range {
            limit: Some(Limit {
                value: SEARCH_PAGE_SIZE,
            }),
            offset: Some(Offset {
                value: SEARCH_PAGE_SIZE * page,
            }),
        }),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().expect("Failed to parse token"),
    );

    client.dispatch_search_request(request).await?;

    Ok(())
}
