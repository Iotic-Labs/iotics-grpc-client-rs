use anyhow::Context;
use std::sync::Arc;
use tonic::transport::Channel;

use crate::client::iotics::api::create_feed_request::{
    Arguments as CreateFeedRequestArguments, Payload as CreateFeedRequestPayload,
};
use crate::client::iotics::api::create_twin_request::Payload as CreateTwinRequestPayload;
use crate::client::iotics::api::delete_twin_request::Arguments as DeleteTwinRequestArguments;
use crate::client::iotics::api::update_feed_request::{
    Arguments as UpdateFeedRequestArguments, Payload as UpdateFeedRequestPayload,
};
use crate::client::iotics::api::update_twin_request::{
    Arguments as UpdateTwinRequestArguments, Payload as UpdateTwinRequestPayload,
};
use crate::client::iotics::api::{
    CreateFeedRequest, CreateTwinRequest, DeleteTwinRequest, Feed, FeedId, GeoLocation,
    GeoLocationUpdate, Headers, LabelUpdate, LangLiteral, Property, PropertyUpdate, Tags, TwinId,
    UpdateFeedRequest, UpdateTwinRequest, Values as FeedValues, Visibility, VisibilityUpdate,
};

use crate::auth_builder::IntoAuthBuilder;
use crate::helpers::generate_client_app_id;

use super::{
    create_feed_api_client, create_twin_api_client, FeedApiClient, TwinApiClient, TwinFeed,
};

pub async fn create_update_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    did: &str,
    label: &str,
    properties: Vec<Property>,
    tags: Vec<String>,
    location: Option<GeoLocation>,
) -> Result<(), anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    create_update_twin_with_client(
        auth_builder,
        &mut client,
        did,
        label,
        properties,
        tags,
        location,
    )
    .await
}

pub async fn create_update_twin_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
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

    let token = auth_builder.get_token()?;

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

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client.update_twin(request).await?;

    Ok(())
}

pub async fn update_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    did: &str,
    properties: Vec<Property>,
    cleared_all: bool,
) -> Result<(), anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    update_twin_with_client(auth_builder, &mut client, did, properties, cleared_all).await
}

pub async fn update_twin_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
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

    let payload = UpdateTwinRequestPayload {
        properties: Some(PropertyUpdate {
            cleared_all,
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

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client.update_twin(request).await?;

    Ok(())
}

pub async fn create_update_feed(
    auth_builder: Arc<impl IntoAuthBuilder>,
    did: &str,
    feed: &TwinFeed,
) -> Result<(), anyhow::Error> {
    let mut client = create_feed_api_client(auth_builder.clone()).await?;

    create_update_feed_with_client(auth_builder, &mut client, did, feed).await
}

pub async fn create_update_feed_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut FeedApiClient<Channel>,
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

    let token = auth_builder.get_token()?;

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

    let token = auth_builder.get_token()?;

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

pub async fn delete_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    did: &str,
) -> Result<(), anyhow::Error> {
    let mut client = create_twin_api_client(auth_builder.clone()).await?;

    delete_twin_with_client(auth_builder, &mut client, did).await
}

pub async fn delete_twin_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut TwinApiClient<Channel>,
    did: &str,
) -> Result<(), anyhow::Error> {
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

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    client.delete_twin(request).await?;

    Ok(())
}
