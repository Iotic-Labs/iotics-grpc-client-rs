use iotics_grpc_client::IntoAuthBuilder;
use iotics_identity::{create_agent_auth_token, create_twin_did_with_control_delegation, Config};
/// Minimal implementation of the `AuthBuilder` for use in example scripts
use std::sync::{Arc, Mutex};

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
            agent_key_name: parse_env("IOTICS_AGENT_KEY_NAME"),
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
    pub config: Config,
    host_address: String,
    token: Arc<Mutex<Option<String>>>,
}

impl AuthBuilder {
    pub fn new(settings: IoticsSettings) -> Self {
        AuthBuilder {
            config: Config {
                resolver_address: settings.resolver_address,
                token_duration: settings.token_duration as i64,
                user_did: settings.user_did,
                agent_did: settings.agent_did,
                agent_key_name: settings.agent_key_name,
                agent_name: settings.agent_name,
                agent_secret: settings.agent_secret,
            },
            host_address: settings.host_address,
            token: Arc::new(Mutex::new(None)),
        }
    }
}

impl IntoAuthBuilder for AuthBuilder {
    fn get_host(&self) -> Result<String, anyhow::Error> {
        Ok(self.host_address.clone())
    }

    fn get_token(&self) -> Result<String, anyhow::Error> {
        let mut token_lock = self
            .token
            .lock()
            .map_err(|_| anyhow::anyhow!("failed to lock the token mutex"))?;

        if token_lock.is_none() {
            let token = create_agent_auth_token(&self.config)?;
            let token = format!("bearer {}", token);

            token_lock.replace(token);
        }

        let token = token_lock.as_ref().expect("this should never happen");

        Ok(token.clone())
    }
}

#[allow(dead_code)]
pub fn generate_twin_did(config: &Config, name: &str) -> String {
    let twin_name = "#".to_string() + name;
    create_twin_did_with_control_delegation(config, name, &twin_name)
        .expect("Creating twin DID failed.")
}

#[allow(dead_code)]
fn main() {}
