use anyhow::Context;
use std::sync::Arc;
use tonic::transport::Channel;

use crate::client::google::protobuf::BoolValue;
use crate::client::iotics::api::create_feed_request::{
    Arguments as CreateFeedRequestArguments, Payload as CreateFeedRequestPayload,
};
use crate::client::iotics::api::create_twin_request::Payload as CreateTwinRequestPayload;
use crate::client::iotics::api::delete_twin_request::Arguments as DeleteTwinRequestArguments;
use crate::client::iotics::api::feed_api_client::FeedApiClient;
use crate::client::iotics::api::twin_api_client::TwinApiClient;
use crate::client::iotics::api::update_feed_request::{
    Arguments as UpdateFeedRequestArguments, Payload as UpdateFeedRequestPayload,
};
use crate::client::iotics::api::update_twin_request::{
    Arguments as UpdateTwinRequestArguments, Payload as UpdateTwinRequestPayload,
};
use crate::client::iotics::api::{
    CreateFeedRequest, CreateTwinRequest, DeleteTwinRequest, FeedId, GeoLocation,
    GeoLocationUpdate, Headers, Property, PropertyUpdate, TwinId, UpdateFeedRequest,
    UpdateTwinRequest, Value as FeedValue, Values as FeedValues,
};

use crate::auth_builder::IntoAuthBuilder;
use crate::channel::create_channel;
use crate::helpers::generate_client_app_id;

pub async fn create_update_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    properties: Vec<Property>,
    location: Option<GeoLocation>,
) -> Result<(), anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    create_update_twin_with_channel(auth_builder, channel, twin_id, properties, location).await
}

pub async fn create_update_twin_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
    properties: Vec<Property>,
    location: Option<GeoLocation>,
) -> Result<(), anyhow::Error> {
    let mut client = TwinApiClient::new(channel);
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let payload = CreateTwinRequestPayload {
        id: twin_id.to_string(),
    };

    let twin_id = TwinId {
        id: twin_id.to_string(),
        ..Default::default()
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

    client.create_twin(request).await.with_context(|| {
        format!(
            "Creating twin failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;

    let args = UpdateTwinRequestArguments {
        twin_id: Some(twin_id),
    };

    let mut payload = UpdateTwinRequestPayload {
        properties: Some(PropertyUpdate {
            cleared_all: true,
            added: properties,
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

    client.update_twin(request).await.with_context(|| {
        format!(
            "Updating twin failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;

    Ok(())
}

pub async fn update_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    properties: PropertyUpdate,
) -> Result<(), anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    update_twin_with_channel(auth_builder, channel, twin_id, properties).await
}

pub async fn update_twin_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
    properties: PropertyUpdate,
) -> Result<(), anyhow::Error> {
    let mut client = TwinApiClient::new(channel);
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let twin_id = TwinId {
        id: twin_id.to_string(),
        ..Default::default()
    };

    let args = UpdateTwinRequestArguments {
        twin_id: Some(twin_id),
    };

    let payload = UpdateTwinRequestPayload {
        properties: Some(properties),
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

    client.update_twin(request).await.with_context(|| {
        format!(
            "Updating twin failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;

    Ok(())
}

pub async fn create_update_feed(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    feed_id: &str,
    store_last: bool,
    properties: Vec<Property>,
    values: Vec<FeedValue>,
) -> Result<(), anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    create_update_feed_with_channel(
        auth_builder,
        channel,
        twin_id,
        feed_id,
        store_last,
        properties,
        values,
    )
    .await
}

pub async fn create_update_feed_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
    feed_id: &str,
    store_last: bool,
    properties: Vec<Property>,
    values: Vec<FeedValue>,
) -> Result<(), anyhow::Error> {
    let mut client = FeedApiClient::new(channel);

    let feed_id_arg = FeedId {
        id: feed_id.to_string(),
        twin_id: twin_id.to_string(),
        ..Default::default()
    };

    let twin_id = TwinId {
        id: twin_id.to_string(),
        ..Default::default()
    };

    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let args = CreateFeedRequestArguments {
        twin_id: Some(twin_id),
    };

    let payload = CreateFeedRequestPayload {
        id: feed_id.to_string(),
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

    client.create_feed(request).await.with_context(|| {
        format!(
            "Creating feed failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;

    let args = UpdateFeedRequestArguments {
        feed_id: Some(feed_id_arg),
    };

    let payload = UpdateFeedRequestPayload {
        store_last: Some(BoolValue { value: store_last }),
        properties: Some(PropertyUpdate {
            cleared_all: true,
            added: properties,
            ..Default::default()
        }),
        values: Some(FeedValues {
            added: values,
            ..Default::default()
        }),
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

    client.update_feed(request).await.with_context(|| {
        format!(
            "Updating feed failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;

    Ok(())
}

pub async fn delete_twin(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
) -> Result<(), anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    delete_twin_with_channel(auth_builder, channel, twin_id).await
}

pub async fn delete_twin_with_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
) -> Result<(), anyhow::Error> {
    let mut client = TwinApiClient::new(channel);
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let twin_id = TwinId {
        id: twin_id.to_string(),
        ..Default::default()
    };

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
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

    client.delete_twin(request).await.with_context(|| {
        format!(
            "Deleting twin failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;

    Ok(())
}
