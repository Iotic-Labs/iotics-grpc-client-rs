/// Example on how to perform a search and receive the paginated results.
mod auth;
use std::{collections::HashMap, sync::Arc, time::Duration};

use iotics_grpc_client::{
    common::{GeoCircle, GeoLocation, Scope},
    host::get_local_host_id,
    search::{search, Filter},
};
use log::{error, info, LevelFilter};

use auth::{AuthBuilder, IoticsSettings};

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

    let local_host_id_response = get_local_host_id(auth_builder.clone())
        .await
        .expect("Failed to get local host id");

    let local_host_id = local_host_id_response.payload.unwrap().host_id;

    let mut search_stream = search(
        auth_builder.clone(),
        filter,
        Scope::Global,
        Some(Duration::from_secs(10)),
    )
    .await
    .expect("search request failed");

    let mut pages_found: HashMap<String, u32> = HashMap::new();

    while let Some(response_result) = search_stream.recv().await {
        match response_result {
            Ok(page) => {
                if let Some(payload) = page.payload {
                    let host_str = if payload.host_id == local_host_id {
                        format!("host {} localhost", payload.host_id)
                    } else {
                        format!("host {} ", payload.host_id)
                    };
                    let page = pages_found
                        .entry(host_str.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    if !payload.twins.is_empty() {
                        info!(
                            "{} twins found on page {} of {} results",
                            payload.twins.len(),
                            page,
                            host_str
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
