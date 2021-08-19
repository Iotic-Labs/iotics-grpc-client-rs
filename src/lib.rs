mod helpers;

#[allow(clippy::module_inception)]
mod api {
    pub mod common;
    pub mod feed;
    pub mod interest;
    pub mod search;
    pub mod twin;

    #[path = ""]
    pub mod google {
        #[path = "google.rpc.rs"]
        pub mod rpc;

        #[path = "google.protobuf.rs"]
        pub mod protobuf;
    }
}

const PAGE_SIZE: u32 = 100;

use anyhow::Context;
use prost_types::Timestamp;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver};
pub use tonic::transport::Channel;
pub use tonic::Streaming;

pub use api::common::{
    property::Value, FeedData, FeedId, GeoCircle, GeoLocation, Headers, HostId, LabelUpdate,
    LangLiteral, Limit, Literal, Offset, Property, Range, Scope, StringLiteral,
    SubscriptionHeaders, Tags, TwinId, Uri, Value as FeedValue, Values as FeedValues, Visibility,
};
use api::feed::create_feed_request::{
    Arguments as CreateFeedRequestArguments, Payload as CreateFeedRequestPayload,
};
pub use api::feed::feed_api_client::FeedApiClient;
use api::feed::share_feed_data_request::{
    Arguments as ShareFeedDataRequestArguments, Payload as ShareFeedDataRequestPayload,
};
use api::feed::update_feed_request::{
    Arguments as UpdateFeedRequestArguments, Payload as UpdateFeedRequestPayload,
};
use api::feed::Feed;
use api::feed::{CreateFeedRequest, ShareFeedDataRequest, UpdateFeedRequest};
use api::interest::fetch_interest_request::Arguments;
use api::interest::interest::FollowedFeed;
pub use api::interest::interest_api_client::InterestApiClient;
use api::interest::Interest;
pub use api::interest::{FetchInterestRequest, FetchInterestResponse};
use api::search::search_api_client::SearchApiClient;
pub use api::search::search_request::payload::Filter;
use api::search::search_request::Payload;
pub use api::search::search_response::TwinDetails;
pub use api::search::{ResponseType, SearchRequest, SearchResponse};
use api::twin::create_twin_request::Payload as CreateTwinRequestPayload;
use api::twin::delete_twin_request::Arguments as DeleteTwinRequestArguments;
use api::twin::twin_api_client::TwinApiClient;
use api::twin::update_twin_request::{
    Arguments as UpdateTwinRequestArguments, Payload as UpdateTwinRequestPayload,
};
use api::twin::{
    CreateTwinRequest, DeleteTwinRequest, GeoLocationUpdate, PropertyUpdate, UpdateTwinRequest,
    VisibilityUpdate,
};

use crate::helpers::generate_client_app_id;

#[derive(Debug, Clone)]
pub struct TwinFeed {
    pub id: String,
    pub label: String,
    pub values: Vec<FeedValue>,
}

#[allow(clippy::too_many_arguments)]
pub async fn create_update_twin_with_feeds(
    host_address: &str,
    token: &str,
    did: &str,
    label: &str,
    properties: Vec<Property>,
    tags: Vec<String>,
    feeds: Vec<TwinFeed>,
    location: Option<GeoLocation>,
) -> Result<(), anyhow::Error> {
    create_update_twin(host_address, token, did, label, properties, tags, location).await?;

    for feed in feeds {
        create_update_feed(host_address, token, did, &feed).await?;
    }

    Ok(())
}

pub async fn share_data_with_channel(
    client: &mut FeedApiClient<Channel>,
    token: &str,
    twin_did: &str,
    feed_id: &str,
    data: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let twin_id = TwinId {
        value: twin_did.to_string(),
    };

    let feed_id = FeedId {
        value: feed_id.to_string(),
    };

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let args = ShareFeedDataRequestArguments {
        feed: Some(Feed {
            id: Some(feed_id),
            twin_id: Some(twin_id),
        }),
    };

    let payload = ShareFeedDataRequestPayload {
        sample: Some(FeedData {
            occurred_at: Some(Timestamp::from(std::time::SystemTime::now())),
            mime: "application/json".to_string(),
            data,
        }),
    };

    let mut request = tonic::Request::new(ShareFeedDataRequest {
        headers: Some(headers.clone()),
        args: Some(args),
        payload: Some(payload),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client
        .share_feed_data(request)
        .await
        .context("share data failed")?;

    Ok(())
}

pub async fn share_data(
    host_address: &str,
    token: &str,
    twin_did: &str,
    feed_id: &str,
    data: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let mut client = FeedApiClient::connect(host_address.to_string()).await?;

    share_data_with_channel(&mut client, token, twin_did, feed_id, data).await
}

pub async fn create_update_feed(
    host_address: &str,
    token: &str,
    did: &str,
    feed: &TwinFeed,
) -> Result<(), anyhow::Error> {
    let mut client = FeedApiClient::connect(host_address.to_string()).await?;

    let twin_id = TwinId {
        value: did.to_string(),
    };

    let feed_id = FeedId {
        value: feed.id.to_string(),
    };

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let args = CreateFeedRequestArguments {
        twin_id: Some(twin_id.clone()),
    };

    let payload = CreateFeedRequestPayload {
        feed_id: Some(feed_id.clone()),
        store_last: true,
    };

    let mut request = tonic::Request::new(CreateFeedRequest {
        headers: Some(headers.clone()),
        args: Some(args),
        payload: Some(payload),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client
        .create_feed(request)
        .await
        .context("create feed failed")?;

    let args = UpdateFeedRequestArguments {
        feed: Some({
            Feed {
                id: Some(feed_id),
                twin_id: Some(twin_id),
            }
        }),
    };

    let payload = UpdateFeedRequestPayload {
        store_last: Some(true),
        labels: Some(LabelUpdate {
            added: vec![LangLiteral {
                lang: "en".to_string(),
                value: feed.label.to_string(),
            }],
            ..Default::default()
        }),
        values: Some(FeedValues {
            added: feed.values.clone(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut request = tonic::Request::new(UpdateFeedRequest {
        headers: Some(headers),
        args: Some(args),
        payload: Some(payload),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client
        .update_feed(request)
        .await
        .context("update feed failed")?;

    Ok(())
}

pub async fn create_update_twin(
    host_address: &str,
    token: &str,
    did: &str,
    label: &str,
    properties: Vec<Property>,
    tags: Vec<String>,
    location: Option<GeoLocation>,
) -> Result<String, anyhow::Error> {
    let mut client = TwinApiClient::connect(host_address.to_string()).await?;

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let twin_id = TwinId {
        value: did.to_string(),
    };

    let payload = CreateTwinRequestPayload {
        twin_id: Some(twin_id.clone()),
    };

    let mut request = tonic::Request::new(CreateTwinRequest {
        headers: Some(headers.clone()),
        payload: Some(payload),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client
        .create_twin(request)
        .await
        .context("create twin failed")?;

    let args = UpdateTwinRequestArguments {
        twin_id: Some(twin_id.clone()),
    };

    let mut payload = UpdateTwinRequestPayload {
        labels: Some(LabelUpdate {
            added: vec![LangLiteral {
                lang: "en".to_string(),
                value: label.to_string(),
            }],
            ..Default::default()
        }),
        new_visibility: Some(VisibilityUpdate {
            visibility: Visibility::Public as i32,
        }),
        properties: Some(PropertyUpdate {
            cleared_all: true,
            added: properties,
            ..Default::default()
        }),
        tags: Some(Tags {
            added: tags,
            ..Default::default()
        }),
        ..Default::default()
    };

    if let Some(location) = location {
        payload.location = Some(GeoLocationUpdate {
            location: Some(location),
        });
    }

    let mut request = tonic::Request::new(UpdateTwinRequest {
        headers: Some(headers),
        args: Some(args),
        payload: Some(payload),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client
        .update_twin(request)
        .await
        .context("update twin failed")?;

    Ok(did.to_string())
}

pub async fn delete_twin(host_address: &str, token: &str, did: &str) -> Result<(), anyhow::Error> {
    let mut client = TwinApiClient::connect(host_address.to_string()).await?;

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let twin_id = TwinId {
        value: did.to_string(),
    };

    let headers = Headers {
        client_app_id,
        transaction_ref,
        ..Default::default()
    };

    let args = DeleteTwinRequestArguments {
        twin_id: Some(twin_id),
    };

    let mut request = tonic::Request::new(DeleteTwinRequest {
        headers: Some(headers),
        args: Some(args),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client
        .delete_twin(request)
        .await
        .context("delete twin failed")?;

    Ok(())
}

pub async fn search(
    host_address: &str,
    token: &str,
    filter: Filter,
) -> Result<Receiver<SearchResponse>, anyhow::Error> {
    let client = SearchApiClient::connect(host_address.to_string()).await?;

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let (tx, rx) = channel::<SearchResponse>(10000);

    let results_client = client.clone();
    let results_token = token.to_string();
    let results_client_app_id = client_app_id.clone();
    let results_transaction_ref = transaction_ref.clone();
    let results_filter = filter.clone();

    let page = Arc::new(AtomicU32::new(0));

    // TODO: rework this so that it returns the error instead of panicking
    let fut = async move {
        let mut request = tonic::Request::new(SubscriptionHeaders {
            client_app_id: results_client_app_id.clone(),
            transaction_ref: results_transaction_ref.clone(),
            ..Default::default()
        });

        request.metadata_mut().append(
            "authorization",
            results_token
                .clone()
                .parse()
                .expect("Failed to parse token"),
        );

        let mut stream = results_client
            .clone()
            .receive_all_search_responses(request)
            .await
            .expect("failed to open the results stream")
            .into_inner();

        while let Ok(Some(result)) = stream.message().await {
            if let Some(payload) = &result.payload {
                if payload.twins.len() >= PAGE_SIZE as usize {
                    let current_page = page.load(Ordering::SeqCst);

                    if result.headers.as_ref().unwrap().client_ref
                        == format!("{}_{}", &results_client_app_id, current_page)
                    {
                        search_page(
                            results_client.clone(),
                            &results_token,
                            results_filter.clone(),
                            results_client_app_id.clone(),
                            results_transaction_ref.clone(),
                            current_page + 1,
                        )
                        .await
                        .expect("failed to request the next page");

                        page.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }

            tx.send(result)
                .await
                .expect("failed to stream back the result");
        }
    };

    tokio::spawn(fut);

    search_page(client, token, filter, client_app_id, transaction_ref, 0).await?;

    Ok(rx)
}

async fn search_page(
    mut client: SearchApiClient<Channel>,
    token: &str,
    filter: Filter,
    client_app_id: String,
    transaction_ref: Vec<String>,
    page: u32,
) -> Result<(), anyhow::Error> {
    let headers = Headers {
        client_app_id: client_app_id.clone(),
        client_ref: format!("{}_{}", client_app_id, page),
        transaction_ref,
        ..Default::default()
    };

    let payload = Payload {
        filter: Some(filter),
        response_type: ResponseType::Full as i32,
        ..Default::default()
    };

    let mut request = tonic::Request::new(SearchRequest {
        lang: Some("en".to_string()),
        scope: Scope::Global as i32,
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
        .context("failed to dispatch the search request")?;

    Ok(())
}

pub async fn follow(
    host_address: &str,
    token: &str,
    followed_host_id: Option<HostId>,
    followed_twin_id: TwinId,
    followed_feed: String,
    follower_twin_id: TwinId,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let mut client = InterestApiClient::connect(host_address.to_string()).await?;

    follow_with_channel(
        &mut client,
        token,
        followed_host_id,
        followed_twin_id,
        followed_feed,
        follower_twin_id,
    )
    .await
}

pub async fn follow_with_channel(
    client: &mut InterestApiClient<Channel>,
    token: &str,
    followed_host_id: Option<HostId>,
    followed_twin_id: TwinId,
    followed_feed: String,
    follower_twin_id: TwinId,
) -> Result<Streaming<FetchInterestResponse>, anyhow::Error> {
    let client_app_id = generate_client_app_id();

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: vec![client_app_id],
        ..Default::default()
    };

    let mut request = tonic::Request::new(FetchInterestRequest {
        headers: Some(headers),
        args: Some(Arguments {
            interest: Some(Interest {
                followed_feed: Some(FollowedFeed {
                    feed: Some(Feed {
                        id: Some(FeedId {
                            value: followed_feed,
                        }),
                        twin_id: Some(followed_twin_id),
                    }),
                    host_id: followed_host_id,
                }),
                follower_twin_id: Some(follower_twin_id),
            }),
        }),
        ..Default::default()
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let stream = client
        .fetch_interests(request)
        .await
        .context("follow feed failed")?
        .into_inner();

    Ok(stream)
}
