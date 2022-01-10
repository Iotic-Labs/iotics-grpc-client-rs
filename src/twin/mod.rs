pub mod upsert;

// TODO extract this into separate files
use anyhow::Context;
use prost_types::Timestamp;
use tonic::transport::Channel;
use tonic::Code;

use crate::client::iotics::api::create_feed_request::{
    Arguments as CreateFeedRequestArguments, Payload as CreateFeedRequestPayload,
};
use crate::client::iotics::api::describe_feed_request::Arguments as DescribeFeedRequestArguments;
use crate::client::iotics::api::share_feed_data_request::{
    Arguments as ShareFeedDataRequestArguments, Payload as ShareFeedDataRequestPayload,
};
use crate::client::iotics::api::update_feed_request::{
    Arguments as UpdateFeedRequestArguments, Payload as UpdateFeedRequestPayload,
};
use crate::client::iotics::api::Feed;
use crate::client::iotics::api::{
    CreateFeedRequest, DescribeFeedRequest, ShareFeedDataRequest, UpdateFeedRequest,
};
use crate::client::iotics::api::{
    FeedData, FeedId, GeoLocation, Headers, HostId, LabelUpdate, LangLiteral, Property, Tags,
    TwinId, Value as FeedValue, Values as FeedValues, Visibility,
};

use crate::client::iotics::api::create_twin_request::Payload as CreateTwinRequestPayload;
use crate::client::iotics::api::delete_twin_request::Arguments as DeleteTwinRequestArguments;
use crate::client::iotics::api::describe_twin_request::Arguments as DescribeTwinRequestArguments;
use crate::client::iotics::api::update_twin_request::{
    Arguments as UpdateTwinRequestArguments, Payload as UpdateTwinRequestPayload,
};
use crate::client::iotics::api::{
    CreateTwinRequest, DeleteTwinRequest, DescribeTwinRequest, GeoLocationUpdate,
    ListAllTwinsRequest, PropertyUpdate, UpdateTwinRequest, VisibilityUpdate,
};

pub use crate::client::iotics::api::feed_api_client::FeedApiClient;
pub use crate::client::iotics::api::twin_api_client::TwinApiClient;
pub use crate::client::iotics::api::DescribeFeedResponse;
pub use crate::client::iotics::api::{DescribeTwinResponse, ListAllTwinsResponse, Twin};
pub use crate::client::iotics::api::{FetchInterestRequest, FetchInterestResponse};

use crate::helpers::generate_client_app_id;

#[derive(Debug, Clone)]
pub struct TwinFeed {
    pub id: String,
    pub label: String,
    pub values: Vec<FeedValue>,
}

pub async fn create_twin_api_client(
    host_address: &str,
) -> Result<TwinApiClient<Channel>, anyhow::Error> {
    let client = TwinApiClient::connect(host_address.to_string()).await?;

    Ok(client)
}

pub async fn create_feed_api_client(
    host_address: &str,
) -> Result<FeedApiClient<Channel>, anyhow::Error> {
    let client = FeedApiClient::connect(host_address.to_string()).await?;

    Ok(client)
}

#[allow(clippy::too_many_arguments)]
pub async fn create_update_twin_with_feeds(
    twin_client: &mut TwinApiClient<Channel>,
    feed_client: &mut FeedApiClient<Channel>,
    token: &str,
    did: &str,
    label: &str,
    properties: Vec<Property>,
    tags: Vec<String>,
    feeds: Vec<TwinFeed>,
    location: Option<GeoLocation>,
) -> Result<(), anyhow::Error> {
    create_update_twin(twin_client, token, did, label, properties, tags, location).await?;

    for feed in feeds {
        create_update_feed(feed_client, token, did, &feed).await?;
    }

    Ok(())
}

pub async fn create_update_twin(
    client: &mut TwinApiClient<Channel>,
    token: &str,
    did: &str,
    label: &str,
    properties: Vec<Property>,
    tags: Vec<String>,
    location: Option<GeoLocation>,
) -> Result<(), anyhow::Error> {
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

    client.create_twin(request).await?;

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

    client.update_twin(request).await?;

    Ok(())
}

pub async fn update_twin(
    client: &mut TwinApiClient<Channel>,
    token: &str,
    did: &str,
    properties: Vec<Property>,
    cleared_all: bool,
) -> Result<(), anyhow::Error> {
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

    let args = UpdateTwinRequestArguments {
        twin_id: Some(twin_id.clone()),
    };

    let property_keys = properties.clone().into_iter().map(|p| p.key).collect();

    let payload = UpdateTwinRequestPayload {
        properties: Some(PropertyUpdate {
            cleared_all,
            deleted_by_key: property_keys,
            added: properties,
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut request = tonic::Request::new(UpdateTwinRequest {
        headers: Some(headers),
        args: Some(args),
        payload: Some(payload),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client.update_twin(request).await?;

    Ok(())
}

pub async fn create_update_feed(
    client: &mut FeedApiClient<Channel>,
    token: &str,
    did: &str,
    feed: &TwinFeed,
) -> Result<(), anyhow::Error> {
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

pub async fn delete_twin(host_address: &str, token: &str, did: &str) -> Result<(), anyhow::Error> {
    let mut client = create_twin_api_client(host_address).await?;

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

    client.delete_twin(request).await?;

    Ok(())
}

pub async fn list_all_twins(
    host_address: &str,
    token: &str,
) -> Result<ListAllTwinsResponse, anyhow::Error> {
    let mut client = create_twin_api_client(host_address).await?;

    list_all_twins_with_client(&mut client, token).await
}

pub async fn list_all_twins_with_client(
    client: &mut TwinApiClient<Channel>,
    token: &str,
) -> Result<ListAllTwinsResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let mut request = tonic::Request::new(ListAllTwinsRequest {
        headers: Some(headers.clone()),
        range: None,
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.list_all_twins(request).await?;

    let result = result.into_inner();

    Ok(result)
}

pub async fn share_data(
    host_address: &str,
    token: &str,
    twin_did: &str,
    feed_id: &str,
    data: Vec<u8>,
    retry_unknown: bool,
) -> Result<(), anyhow::Error> {
    let mut client = create_feed_api_client(host_address).await?;

    share_data_with_client(&mut client, token, twin_did, feed_id, data, retry_unknown).await
}

pub async fn share_data_with_client(
    client: &mut FeedApiClient<Channel>,
    token: &str,
    twin_did: &str,
    feed_id: &str,
    data: Vec<u8>,
    retry_unknown: bool,
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
        args: Some(args.clone()),
        payload: Some(payload.clone()),
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.share_feed_data(request).await;

    if let Err(e) = result {
        if retry_unknown && e.code() == Code::Unknown {
            let mut request = tonic::Request::new(ShareFeedDataRequest {
                headers: Some(headers.clone()),
                args: Some(args),
                payload: Some(payload),
            });

            request.metadata_mut().append(
                "authorization",
                token.parse().context("parse token failed")?,
            );

            client.share_feed_data(request).await?;
        } else {
            return Err(e.into());
        }
    }

    Ok(())
}

pub async fn describe_twin_with_client(
    client: &mut TwinApiClient<Channel>,
    token: &str,
    twin_id: TwinId,
    remote_host_id: Option<HostId>,
) -> Result<DescribeTwinResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let args = DescribeTwinRequestArguments {
        twin_id: Some(twin_id),
        remote_host_id,
    };

    let mut request = tonic::Request::new(DescribeTwinRequest {
        headers: Some(headers.clone()),
        args: Some(args),
        lang: None,
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.describe_twin(request).await?;

    let result = result.into_inner();

    Ok(result)
}

pub async fn describe_feed_with_client(
    client: &mut FeedApiClient<Channel>,
    token: &str,
    twin_id: TwinId,
    feed_id: FeedId,
    remote_host_id: Option<HostId>,
) -> Result<DescribeFeedResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id: client_app_id.clone(),
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let args = DescribeFeedRequestArguments {
        feed: Some(Feed {
            id: Some(feed_id),
            twin_id: Some(twin_id),
        }),
        remote_host_id,
    };

    let mut request = tonic::Request::new(DescribeFeedRequest {
        headers: Some(headers.clone()),
        args: Some(args),
        lang: None,
    });

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.describe_feed(request).await?;

    let result = result.into_inner();

    Ok(result)
}
