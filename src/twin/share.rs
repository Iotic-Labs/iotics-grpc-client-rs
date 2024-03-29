use anyhow::Context;
use std::sync::Arc;
use std::time::SystemTime;
use tonic::transport::Channel;
use tonic::Code;

use crate::client::google::protobuf::Timestamp;
use crate::client::iotics::api::feed_api_client::FeedApiClient;
use crate::client::iotics::api::share_feed_data_request::{
    Arguments as ShareFeedDataRequestArguments, Payload as ShareFeedDataRequestPayload,
};
use crate::client::iotics::api::{FeedData, FeedId, Headers, ShareFeedDataRequest};

use crate::auth_builder::IntoAuthBuilder;
use crate::channel::create_channel;
use crate::helpers::generate_client_app_id;

pub async fn share_data<T: Into<Vec<u8>>>(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    feed_id: &str,
    data: T,
    retry_unknown: bool,
) -> Result<(), anyhow::Error> {
    let channel = create_channel(auth_builder.clone(), None, None, None).await?;
    share_data_with_channel(auth_builder, channel, twin_id, feed_id, data, retry_unknown).await
}

pub async fn share_data_with_channel<T: Into<Vec<u8>>>(
    auth_builder: Arc<impl IntoAuthBuilder>,
    channel: Channel,
    twin_id: &str,
    feed_id: &str,
    data: T,
    retry_unknown: bool,
) -> Result<(), anyhow::Error> {
    let mut client = FeedApiClient::new(channel);
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let args = ShareFeedDataRequestArguments {
        feed_id: Some(FeedId {
            id: feed_id.to_string(),
            twin_id: twin_id.to_string(),
            ..Default::default()
        }),
    };

    let dtm = std::time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let timestamp = Timestamp {
        seconds: dtm.as_secs() as i64,
        nanos: 0,
    };

    let payload = ShareFeedDataRequestPayload {
        sample: Some(FeedData {
            occurred_at: Some(timestamp),
            mime: "application/json".to_string(),
            data: data.into(),
        }),
    };

    let mut request = tonic::Request::new(ShareFeedDataRequest {
        headers: Some(headers.clone()),
        args: Some(args.clone()),
        payload: Some(payload.clone()),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.share_feed_data(request).await;

    if let Err(e) = result {
        if retry_unknown && e.code() == Code::Unknown {
            let mut request = tonic::Request::new(ShareFeedDataRequest {
                headers: Some(headers),
                args: Some(args),
                payload: Some(payload),
            });

            request.metadata_mut().append(
                "authorization",
                token.parse().context("parse token failed")?,
            );

            client.share_feed_data(request).await.with_context(|| {
                format!(
                    "Sharing data failed, transaction ref [{}]",
                    transaction_ref.join(", ")
                )
            })?;
        } else {
            return Err(anyhow::Error::new(e).context(format!(
                "Sharing data failed, transaction ref [{}]",
                transaction_ref.join(", ")
            )));
        }
    }

    Ok(())
}
