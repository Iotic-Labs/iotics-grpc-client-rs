mod search {
    tonic::include_proto!("search");
}

mod common {
    tonic::include_proto!("common");
}

mod twin {
    tonic::include_proto!("twin");
}

mod feed {
    tonic::include_proto!("feed");
}

mod interest {
    tonic::include_proto!("interest");
}

mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

mod helpers;

const PAGE_SIZE: u32 = 100;

use anyhow::Context;
use prost_types::Timestamp;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver};
use tonic::transport::Channel;
pub use tonic::Streaming;

pub use common::{
    property::Value, FeedData, FeedId, GeoCircle, GeoLocation, Headers, HostId, LabelUpdate,
    LangLiteral, Limit, Offset, Property, Range, Scope, StringLiteral, SubscriptionHeaders, Tags,
    TwinId, Uri, Value as FeedValue, Values as FeedValues, Visibility,
};
use feed::create_feed_request::{
    Arguments as CreateFeedRequestArguments, Payload as CreateFeedRequestPayload,
};
use feed::feed_api_client::FeedApiClient;
use feed::share_feed_data_request::{
    Arguments as ShareFeedDataRequestArguments, Payload as ShareFeedDataRequestPayload,
};
use feed::update_feed_request::{
    Arguments as UpdateFeedRequestArguments, Payload as UpdateFeedRequestPayload,
};
use feed::Feed;
use feed::{CreateFeedRequest, ShareFeedDataRequest, UpdateFeedRequest};
use interest::fetch_interest_request::Arguments;
use interest::interest::FollowedFeed;
use interest::interest_api_client::InterestApiClient;
use interest::Interest;
pub use interest::{FetchInterestRequest, FetchInterestResponse};
pub use iotics_identity::Config;
use search::search_api_client::SearchApiClient;
pub use search::search_request::payload::Filter;
use search::search_request::Payload;
pub use search::search_response::TwinDetails;
pub use search::{ResponseType, SearchRequest, SearchResponse};
use twin::create_twin_request::Payload as CreateTwinRequestPayload;
use twin::delete_twin_request::Arguments as DeleteTwinRequestArguments;
use twin::twin_api_client::TwinApiClient;
use twin::update_twin_request::{
    Arguments as UpdateTwinRequestArguments, Payload as UpdateTwinRequestPayload,
};
use twin::{
    CreateTwinRequest, DeleteTwinRequest, PropertyUpdate, UpdateTwinRequest, VisibilityUpdate,
};

use crate::helpers::generate_client_app_id;

pub fn get_api_config() -> Config {
    dotenv::dotenv().ok();

    let parse_env =
        |key: &str| -> String { std::env::var(key).expect(&format!("env var {}", key)) };

    let api_config = Config {
        host_address: parse_env("IOTICS_HOST_ADDRESS"),
        resolver_address: parse_env("IOTICS_RESOLVER_ADDRESS"),
        token_duration: parse_env("IOTICS_TOKEN_DURATION")
            .parse::<i64>()
            .expect("Failed to parse duration"),
        user_did: parse_env("IOTICS_USER_DID"),
        agent_did: parse_env("IOTICS_AGENT_DID"),
        agent_key_name: parse_env("IOTICS_AGENT_KEY_NAME"),
        agent_secret: parse_env("IOTICS_AGENT_SECRET"),
    };

    api_config
}

pub fn get_token(config: &Config) -> String {
    let token = iotics_identity::new_authentication_token(&config);
    format!("bearer {}", token)
}

#[derive(Debug, Clone)]
pub struct TwinFeed {
    pub id: String,
    pub label: String,
    pub values: Vec<FeedValue>,
}

pub async fn create_update_twin_with_feeds(
    api_config: &Config,
    token: String,
    seed: String,
    label: String,
    properties: Vec<Property>,
    tags: Vec<String>,
    feeds: Vec<TwinFeed>,
) -> Result<String, anyhow::Error> {
    let did = create_update_twin(api_config, token.clone(), seed, label, properties, tags).await?;

    for feed in feeds {
        create_update_feed(api_config, token.clone(), &did, &feed).await?;
    }

    Ok(did)
}

pub async fn share_data(
    api_config: &Config,
    token: String,
    twin_did: &str,
    feed_id: &str,
    data: Vec<u8>,
) -> Result<(), anyhow::Error> {
    let mut client = FeedApiClient::connect(api_config.host_address.clone()).await?;

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
        ..Default::default()
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

pub async fn create_update_feed(
    api_config: &Config,
    token: String,
    did: &str,
    feed: &TwinFeed,
) -> Result<(), anyhow::Error> {
    let mut client = FeedApiClient::connect(api_config.host_address.clone()).await?;

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
        ..Default::default()
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
        ..Default::default()
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
    api_config: &Config,
    token: String,
    seed: String,
    label: String,
    properties: Vec<Property>,
    tags: Vec<String>,
) -> Result<String, anyhow::Error> {
    let mut client = TwinApiClient::connect(api_config.host_address.clone()).await?;

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let did = iotics_identity::new_twin_did(&api_config, seed)
        .await
        .context("create did failed")?;

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let twin_id = TwinId { value: did.clone() };

    let payload = CreateTwinRequestPayload {
        twin_id: Some(twin_id.clone()),
        ..Default::default()
    };

    let mut request = tonic::Request::new(CreateTwinRequest {
        headers: Some(headers.clone()),
        payload: Some(payload),
        ..Default::default()
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

    let payload = UpdateTwinRequestPayload {
        labels: Some(LabelUpdate {
            added: vec![LangLiteral {
                lang: "en".to_string(),
                value: label,
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

    let mut request = tonic::Request::new(UpdateTwinRequest {
        headers: Some(headers),
        args: Some(args),
        payload: Some(payload),
        ..Default::default()
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client
        .update_twin(request)
        .await
        .context("update twin failed")?;

    Ok(did)
}

pub async fn delete_twin(
    api_config: &Config,
    token: String,
    did: String,
) -> Result<(), anyhow::Error> {
    let mut client = TwinApiClient::connect(api_config.host_address.clone()).await?;

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let twin_id = TwinId { value: did };

    let headers = Headers {
        client_app_id: client_app_id,
        transaction_ref: transaction_ref,
        ..Default::default()
    };

    let args = DeleteTwinRequestArguments {
        twin_id: Some(twin_id),
    };

    let mut request = tonic::Request::new(DeleteTwinRequest {
        headers: Some(headers),
        args: Some(args),
        ..Default::default()
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
    api_config: &Config,
    token: String,
    filter: Filter,
) -> Result<Receiver<SearchResponse>, Box<dyn std::error::Error>> {
    let client = SearchApiClient::connect(api_config.host_address.clone()).await?;

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let (tx, rx) = channel::<SearchResponse>(10000);

    let results_client = client.clone();
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
            .unwrap()
            .into_inner();

        while let Ok(Some(result)) = stream.message().await {
            if let Some(payload) = &result.payload {
                if payload.twins.len() >= PAGE_SIZE as usize {
                    let current_page = page.load(Ordering::SeqCst);

                    if &result.headers.as_ref().unwrap().client_ref
                        == &format!("{}_{}", &results_client_app_id, current_page)
                    {
                        search_page(
                            results_client.clone(),
                            results_token.clone(),
                            results_filter.clone(),
                            results_client_app_id.clone(),
                            results_transaction_ref.clone(),
                            current_page + 1,
                        )
                        .await
                        .unwrap();

                        page.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }

            tx.send(result).await.unwrap();
        }
    };

    tokio::spawn(fut);

    search_page(client, token, filter, client_app_id, transaction_ref, 0)
        .await
        .unwrap();

    Ok(rx)
}

async fn search_page(
    mut client: SearchApiClient<Channel>,
    token: String,
    filter: Filter,
    client_app_id: String,
    transaction_ref: Vec<String>,
    page: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let headers = Headers {
        client_app_id: client_app_id.clone(),
        client_ref: format!("{}_{}", client_app_id, page),
        transaction_ref: transaction_ref,
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
        ..Default::default()
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().expect("Failed to parse token"),
    );

    client.dispatch_search_request(request).await.unwrap();

    Ok(())
}

pub async fn follow(
    api_config: &Config,
    token: String,
    followed_host_id: Option<HostId>,
    followed_twin_id: TwinId,
    followed_feed: String,
    follower_twin_id: TwinId,
) -> Result<Streaming<FetchInterestResponse>, Box<dyn std::error::Error>> {
    let mut client = InterestApiClient::connect(api_config.host_address.clone()).await?;

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
        // fetch_last_stored: Some(true),
        ..Default::default()
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().expect("Failed to parse token"),
    );

    let stream = client.fetch_interests(request).await?.into_inner();

    Ok(stream)
}

// async fn test_describe(host_url: &str, did: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = TwinApiClient::connect(host_url.to_string()).await?;

//     let headers = Headers {
//         client_app_id: "sub-id".to_string(),
//         transaction_ref: vec!["test".to_string()],
//         ..Default::default()
//     };

//     let args = Arguments {
//         twin_id: Some(TwinId {
//             value: did.to_string(),
//         }),
//         ..Default::default()
//     };

//     let mut request = tonic::Request::new(DescribeTwinRequest {
//         lang: Some("en".to_string()),
//         headers: Some(headers),
//         args: Some(args),
//         ..Default::default()
//     });

//     let token = format!("bearer {}", get_token());

//     request
//         .metadata_mut()
//         .append("authorization", token.parse().expect("Failed to parse token"));

//     let result = client.describe_twin(request).await?;
//     println!("result = {:?}", result);

//     Ok(())
// }
