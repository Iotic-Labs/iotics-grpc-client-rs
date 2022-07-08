/// Example on how to perform a search and receive the paginated results.
/// It contains a minimal implementation of the `AuthBuilder`.
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use iotics_grpc_client::{
    auth_builder::IntoAuthBuilder,
    common::{GeoCircle, GeoLocation, Scope},
    search::{search, Filter},
};
use iotics_identity::{create_agent_auth_token, Config};
use log::{error, info, LevelFilter};

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

    // Search in a radius of 25 Km from London's centre
    let filter = Filter {
        properties: Vec::new(),
        location: Some(GeoCircle {
            location: Some(GeoLocation {
                lat: 51.5448574,
                lon: -0.0838615,
            }),
            radius_km: 25.0,
        }),
        text: None,
    };

    let mut search_stream = search(
        auth_builder.clone(),
        filter,
        Scope::Global,
        Some(Duration::from_secs(10)),
    )
    .await
    .expect("search request failed");

    while let Some(response_result) = search_stream.recv().await {
        match response_result {
            Ok(page) => {
                if let Some(payload) = page.payload {
                    if !payload.twins.is_empty() {
                        info!(
                            "{} twins found in host {:?}",
                            payload.twins.len(),
                            payload.remote_host_id
                        )
                    }
                }
            }
            Err(e) => {
                error!("search returned error: {e:?}")
            }
        }
    }
    info!("Timeout reached.");
}

pub const AGENT_KEY_NAME: &str = "00";

#[derive(Clone)]
pub struct IoticsSettings {
    pub host_address: String,
    pub resolver_address: String,
    pub token_duration: usize,
    pub user_did: String,
    pub agent_did: String,
    pub agent_key_name: String,
    pub agent_name: String,
    pub agent_secret: String,
}

impl IoticsSettings {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let parse_env = |key: &str| -> String {
            std::env::var(key).unwrap_or_else(|_| panic!("env var {} is missing", key))
        };

        Self {
            host_address: parse_env("IOTICS_HOST_ADDRESS"),
            resolver_address: parse_env("IOTICS_RESOLVER_ADDRESS"),
            user_did: parse_env("IOTICS_USER_DID"),
            agent_did: parse_env("IOTICS_AGENT_DID"),
            agent_key_name: AGENT_KEY_NAME.to_string(),
            agent_name: format!("#{}", parse_env("IOTICS_AGENT_NAME")),
            agent_secret: parse_env("IOTICS_AGENT_SECRET"),
            token_duration: parse_env("IOTICS_TOKEN_DURATION")
                .parse::<usize>()
                .expect("Failed to parse duration"),
        }
    }
}

impl Default for IoticsSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct AuthBuilder {
    settings: IoticsSettings,
    token: Arc<Mutex<Option<String>>>,
}

impl AuthBuilder {
    pub fn new(settings: IoticsSettings) -> Self {
        AuthBuilder {
            settings,
            token: Arc::new(Mutex::new(None)),
        }
    }
}

impl IntoAuthBuilder for AuthBuilder {
    fn get_host(&self) -> Result<String, anyhow::Error> {
        Ok(self.settings.host_address.clone())
    }

    fn get_token(&self) -> Result<String, anyhow::Error> {
        let mut token_lock = self
            .token
            .lock()
            .map_err(|_| anyhow::anyhow!("failed to lock the token mutex"))?;

        if token_lock.is_none() {
            let identity_config = Config {
                resolver_address: self.settings.resolver_address.clone(),
                token_duration: self.settings.token_duration as i64,
                user_did: self.settings.user_did.clone(),
                agent_did: self.settings.agent_did.clone(),
                agent_key_name: self.settings.agent_key_name.clone(),
                agent_name: self.settings.agent_name.clone(),
                agent_secret: self.settings.agent_secret.clone(),
            };

            let token = create_agent_auth_token(&identity_config)?;
            let token = format!("bearer {}", token);

            token_lock.replace(token);
        }

        let token = token_lock.as_ref().expect("this should never happen");

        Ok(token.clone())
    }
}
