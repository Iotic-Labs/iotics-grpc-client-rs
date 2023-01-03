mod auth;

use serde_json::{from_str, json, Value};
use std::{str::from_utf8, thread::sleep, time::Duration};

use iotics_grpc_client::input::{delete_input, describe_input, receive_input_messages};
use iotics_grpc_client::interest::send_input_message;
use iotics_grpc_client::properties::{common_keys::predicate::COMMENT, PropertyBuilder};
use iotics_grpc_client::twin::{crud::delete_twin, upsert::upsert_twin, UpsertInputWithMeta};
use iotics_grpc_client::FeedValue;
use log::{error, info, LevelFilter};
use std::sync::Arc;

use auth::{generate_twin_did, AuthBuilder, IoticsSettings};

pub const INPUT_NAME: &str = "input";
pub const VALUE_LABEL: &str = "message";

#[tokio::main]
async fn main() {
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter_level(log_level)
        .init();
    let settings = IoticsSettings::new();
    let auth_builder = Arc::new(AuthBuilder::new(settings.clone()));

    let sender_twin_id = generate_twin_did(&auth_builder.config, "sender");
    let receiver_twin_id = generate_twin_did(&auth_builder.config, "receiver");
    info!(
        "Twin {} will be sending to an input called '{}' on twin {}",
        sender_twin_id, INPUT_NAME, receiver_twin_id
    );
    info!("Creating twins...");

    create_sender(auth_builder.clone(), &sender_twin_id).await;
    create_receiver(auth_builder.clone(), &receiver_twin_id).await;
    activate_receiver_thread(auth_builder.clone(), &receiver_twin_id).await;
    send_messages(auth_builder.clone(), &sender_twin_id, &receiver_twin_id).await;
    clean_space(auth_builder.clone(), &sender_twin_id, &receiver_twin_id).await;
}

async fn create_sender(auth_builder: Arc<AuthBuilder>, did: &str) {
    upsert_twin(
        auth_builder,
        did,
        vec![
            PropertyBuilder::build_label("en", "sender twin"),
            PropertyBuilder::build_lang_literal(COMMENT, "en", "rust client example script"),
        ],
        vec![],
        vec![],
        None,
    )
    .await
    .expect("Upserting sender failed");
}

async fn create_receiver(auth_builder: Arc<AuthBuilder>, did: &str) {
    upsert_twin(
        auth_builder.clone(),
        did,
        vec![
            PropertyBuilder::build_label("en", "receiver twin"),
            PropertyBuilder::build_lang_literal(COMMENT, "en", "rust client example script"),
        ],
        vec![],
        vec![UpsertInputWithMeta {
            id: INPUT_NAME.to_string(),
            // TODO: Values are for inputs too, check other libs for safety of renaming struct
            values: vec![FeedValue {
                label: VALUE_LABEL.to_string(),
                comment: "a text string from the sender".to_string(),
                unit: "".to_string(),
                data_type: "byte".to_string(),
            }],
            properties: vec![],
        }],
        None,
    )
    .await
    .expect("Upserting receiver failed");
    // TODO: Make such payloads printable
    describe_input(auth_builder, did, INPUT_NAME, None)
        .await
        .expect("Failed to describe input");
}

async fn activate_receiver_thread(auth_builder: Arc<AuthBuilder>, did: &str) {
    let mut message_stream = receive_input_messages(auth_builder.clone(), did, INPUT_NAME)
        .await
        .expect("Activating receiver failed");
    let fut = async move {
        while let Some(response) = message_stream.recv().await {
            match response {
                Ok(message_data) => {
                    let message_json: Value = from_str(
                        from_utf8(&message_data).expect("UTF8 error decoding input message"),
                    )
                    .expect("Error deserialising message json");

                    let message: &str = message_json["message"].as_str().unwrap();

                    info!("MESSAGE RECEIVED: {}", message);
                    if message == "!" {
                        info!("Last character received!");
                        break;
                    }
                }
                Err(e) => {
                    error!("Input stream returned error: {e:?}")
                }
            }
        }
    };
    tokio::spawn(fut);
}

async fn send_messages(auth_builder: Arc<AuthBuilder>, sender_did: &str, receiver_did: &str) {
    for character in "HELLO, IOTICS!".chars() {
        let message = json!({ VALUE_LABEL: character });
        send_input_message(
            auth_builder.clone(),
            None,
            receiver_did,
            INPUT_NAME,
            sender_did,
            message.to_string().into_bytes(),
        )
        .await
        .unwrap_or_else(|_| panic!("Sending message {} failed", message));
        info!("SENT MESSAGE: {}", character);
        sleep(Duration::from_secs(1))
    }
}

async fn clean_space(auth_builder: Arc<AuthBuilder>, sender_did: &str, receiver_did: &str) {
    info!("Deleting twins...");
    delete_twin(auth_builder.clone(), sender_did)
        .await
        .expect("Deleting sender failed");
    // For demonstrating the method; input would be deleted anyway when twin is deleted immediately afterward.
    delete_input(auth_builder.clone(), receiver_did, INPUT_NAME)
        .await
        .expect("Deleting input failed");
    delete_twin(auth_builder.clone(), receiver_did)
        .await
        .expect("Deleting receiver failed");
}
