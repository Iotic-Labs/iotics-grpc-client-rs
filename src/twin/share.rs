use anyhow::Context;
use std::sync::Arc;
use std::time::SystemTime;
use tonic::Code;

use crate::client::google::protobuf::Timestamp;
use crate::client::iotics::api::share_feed_data_request::{
    Arguments as ShareFeedDataRequestArguments, Payload as ShareFeedDataRequestPayload,
};
use crate::client::iotics::api::ShareFeedDataRequest;

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, FeedData, FeedId, Headers};
use crate::helpers::generate_client_app_id;

use super::{create_feed_api_client, FeedApiClient};

pub async fn share_data<T: Into<Vec<u8>>>(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    feed_id: &str,
    data: T,
    retry_unknown: bool,
) -> Result<(), anyhow::Error> {
    let mut client = create_feed_api_client(auth_builder.clone()).await?;

    share_data_with_client(
        auth_builder,
        &mut client,
        twin_id,
        feed_id,
        data,
        retry_unknown,
    )
    .await
}

pub async fn share_data_with_client<T: Into<Vec<u8>>>(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut FeedApiClient<Channel>,
    twin_id: &str,
    feed_id: &str,
    data: T,
    retry_unknown: bool,
) -> Result<(), anyhow::Error> {
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
