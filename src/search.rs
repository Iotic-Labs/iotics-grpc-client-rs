use anyhow::Context;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Channel;

use crate::auth_builder::IntoAuthBuilder;
use crate::channel::create_channel;
use crate::client::google::protobuf::StringValue;
use crate::client::iotics::api::search_api_client::SearchApiClient;
use crate::client::iotics::api::search_request::Payload as SearchRequestPayload;
use crate::client::iotics::api::{Headers, Limit, Offset, Range, Scope, SubscriptionHeaders};
use crate::helpers::generate_client_app_id;
use crate::twin::PAGE_SIZE;

pub use crate::client::iotics::api::search_request::payload::Filter;
pub use crate::client::iotics::api::search_response::{
    FeedDetails, Payload as SearchResponsePayload, TwinDetails,
};
pub use crate::client::iotics::api::{ResponseType, SearchRequest, SearchResponse};

pub async fn search(
    auth_builder: Arc<impl IntoAuthBuilder>,
    filter: Filter,
    scope: Scope,
    timeout: Option<Duration>,
) -> Result<mpsc::Receiver<Result<SearchResponse, anyhow::Error>>, anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    search_with_channel(auth_builder, channel, filter, scope, timeout).await
}

pub async fn search_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    filter: Filter,
    scope: Scope,
    timeout: Option<Duration>,
) -> Result<mpsc::Receiver<Result<SearchResponse, anyhow::Error>>, anyhow::Error> {
    let mut client = SearchApiClient::new(channel.clone());
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let token = auth_builder.get_token()?;

    let (tx, rx) = mpsc::channel::<Result<SearchResponse, anyhow::Error>>(16384);

    let mut results_client = client.clone();
    let results_token = token.clone();
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
                                if payload.twins.len() >= PAGE_SIZE as usize {
                                    let current_page = page.load(Ordering::SeqCst);

                                    if result
                                        .headers
                                        .as_ref()
                                        .expect("this should not happen")
                                        .client_ref
                                        == format!("{}_{}", &results_client_app_id, current_page)
                                    {
                                        let response = search_page_with_client(
                                            &mut results_client,
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

    search_page_with_client(
        &mut client,
        &token,
        filter,
        scope,
        0,
        client_app_id,
        transaction_ref,
    )
    .await?;

    Ok(rx)
}

async fn search_page_with_client(
    client: &mut SearchApiClient<Channel>,
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
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let payload = SearchRequestPayload {
        filter: Some(filter),
        response_type: ResponseType::Full as i32,
        ..Default::default()
    };

    let mut request = tonic::Request::new(SearchRequest {
        lang: Some(StringValue {
            value: "en".to_string(),
        }),
        scope: scope as i32,
        payload: Some(payload),
        headers: Some(headers),
        range: Some(Range {
            limit: Some(Limit { value: PAGE_SIZE }),
            offset: Some(Offset {
                value: PAGE_SIZE * page,
            }),
        }),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().expect("Failed to parse token"),
    );

    client
        .dispatch_search_request(request)
        .await
        .with_context(|| {
            format!(
                "Search request failed, transaction ref [{}]",
                transaction_ref.join(", ")
            )
        })?;

    Ok(())
}
