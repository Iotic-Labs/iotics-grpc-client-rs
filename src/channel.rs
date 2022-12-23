use std::sync::Arc;
use std::time::Duration;

use tonic::transport::{Channel, Endpoint};

use crate::auth_builder::IntoAuthBuilder;

/// Create a [`tonic::transport::Channel`] that can be re-used between multiple calls.
/// It provides a `Clone` implementation that is _cheap_.
///
/// # Arguments
///
/// - `auth_builder` - [`crate::IntoAuthBuilder`] implementation that provides the authentication
/// - `concurrency_limit` - The maximum number of concurrent requests that can be made on the channel
/// - `rate_limit` - The maximum number of requests that can be made over a specific interval
/// - `keep_alive_interval` - The interval at which keep alive pings are sent
pub async fn create_channel(
    auth_builder: Arc<impl IntoAuthBuilder>,
    concurrency_limit: Option<usize>,
    rate_limit: Option<(u64, Duration)>,
    keep_alive_interval: Option<Duration>,
) -> Result<Channel, anyhow::Error> {
    let host_address = auth_builder.get_host()?;

    let mut endpoint = Endpoint::new(host_address)?;

    if let Some(concurrency_limit) = concurrency_limit {
        endpoint = endpoint.concurrency_limit(concurrency_limit);
    }

    if let Some((limit, duration)) = rate_limit {
        endpoint = endpoint.rate_limit(limit, duration);
    }

    if let Some(keep_alive_interval) = keep_alive_interval {
        endpoint = endpoint.http2_keep_alive_interval(keep_alive_interval);
    }

    let channel = endpoint.connect().await?;

    Ok(channel)
}
