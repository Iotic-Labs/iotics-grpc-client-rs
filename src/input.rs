use anyhow::Context;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver};

pub use crate::client::iotics::api::input_api_client::InputApiClient;
use crate::client::iotics::api::{
    delete_input_request, describe_input_request, receive_input_message_request,
    DeleteInputResponse, DescribeInputResponse,
};
pub use crate::client::iotics::api::{
    DeleteInputRequest, DescribeInputRequest, ReceiveInputMessageRequest,
    ReceiveInputMessageResponse,
};

use crate::auth_builder::IntoAuthBuilder;
use crate::common::{Channel, Headers, InputId};
use crate::helpers::generate_client_app_id;

pub async fn create_input_api_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
) -> Result<InputApiClient<Channel>, anyhow::Error> {
    let host_address = auth_builder.get_host()?;
    let client = InputApiClient::connect(host_address).await?;

    Ok(client)
}

pub async fn receive_input_messages(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    input_id: &str,
) -> Result<Receiver<Result<Vec<u8>, anyhow::Error>>, anyhow::Error> {
    let mut client = create_input_api_client(auth_builder.clone()).await?;

    receive_input_messages_with_client(auth_builder, &mut client, twin_id, input_id).await
}

pub async fn receive_input_messages_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut InputApiClient<Channel>,
    twin_id: &str,
    input_id: &str,
) -> Result<Receiver<Result<Vec<u8>, anyhow::Error>>, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref,
        ..Default::default()
    };

    let mut request = tonic::Request::new(ReceiveInputMessageRequest {
        headers: Some(headers),
        args: Some(receive_input_message_request::Arguments {
            input_id: Some(InputId {
                id: input_id.to_string(),
                twin_id: twin_id.to_string(),
                ..Default::default()
            }),
        }),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let mut results_client = client.clone();
    let (tx, rx) = channel::<Result<Vec<u8>, anyhow::Error>>(16384);

    let fut = async move {
        let stream = results_client.receive_input_messages(request).await;

        match stream {
            Ok(mut stream) => {
                let stream = stream.get_mut();

                while let Ok(Some(response)) = stream.message().await {
                    match response.payload {
                        Some(payload) => {
                            match payload.message {
                                Some(message) => {
                                    if tx.send(Ok(message.data)).await.is_err() {
                                        break;
                                    }
                                }
                                None => {
                                    if tx
                                        .send(Err(anyhow::anyhow!("Empty input payload")))
                                        .await
                                        .is_err()
                                    {
                                        break;
                                    }
                                }
                            };
                        }
                        None => {
                            if tx
                                .send(Err(anyhow::anyhow!("Empty input response")))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    };
                }
            }
            Err(e) => {
                let _ = tx.send(Err(e.into())).await;
            }
        }
    };

    tokio::spawn(fut);
    Ok(rx)
}

pub async fn describe_input(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    input_id: &str,
    remote_host_id: Option<&str>,
) -> Result<DescribeInputResponse, anyhow::Error> {
    let mut client = create_input_api_client(auth_builder.clone()).await?;

    describe_input_with_client(auth_builder, &mut client, twin_id, input_id, remote_host_id).await
}

pub async fn describe_input_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut InputApiClient<Channel>,
    twin_id: &str,
    input_id: &str,
    remote_host_id: Option<&str>,
) -> Result<DescribeInputResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let mut request = tonic::Request::new(DescribeInputRequest {
        headers: Some(headers),
        args: Some(describe_input_request::Arguments {
            input_id: Some(InputId {
                id: input_id.to_string(),
                twin_id: twin_id.to_string(),
                host_id: remote_host_id.unwrap_or_default().to_string(),
            }),
        }),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.describe_input(request).await.with_context(|| {
        format!(
            "Describing input failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;
    let result = result.into_inner();

    Ok(result)
}

pub async fn delete_input(
    auth_builder: Arc<impl IntoAuthBuilder>,
    twin_id: &str,
    input_id: &str,
) -> Result<DeleteInputResponse, anyhow::Error> {
    let mut client = create_input_api_client(auth_builder.clone()).await?;

    delete_input_with_client(auth_builder, &mut client, twin_id, input_id).await
}

pub async fn delete_input_with_client(
    auth_builder: Arc<impl IntoAuthBuilder>,
    client: &mut InputApiClient<Channel>,
    twin_id: &str,
    input_id: &str,
) -> Result<DeleteInputResponse, anyhow::Error> {
    let client_app_id = generate_client_app_id();
    let transaction_ref = vec![client_app_id.clone()];

    let headers = Headers {
        client_app_id,
        transaction_ref: transaction_ref.clone(),
        ..Default::default()
    };

    let mut request = tonic::Request::new(DeleteInputRequest {
        headers: Some(headers),
        args: Some(delete_input_request::Arguments {
            input_id: Some(InputId {
                id: input_id.to_string(),
                twin_id: twin_id.to_string(),
                ..Default::default()
            }),
        }),
    });

    let token = auth_builder.get_token()?;

    request.metadata_mut().append(
        "authorization",
        token.parse().context("parse token failed")?,
    );

    let result = client.delete_input(request).await.with_context(|| {
        format!(
            "Deleting input failed, transaction ref [{}]",
            transaction_ref.join(", ")
        )
    })?;
    let result = result.into_inner();

    Ok(result)
}
