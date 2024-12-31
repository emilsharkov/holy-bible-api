use std::{convert::Infallible, sync::Arc};
use axum::routing::Route;
use governor::{clock::QuantaInstant, middleware::NoOpMiddleware};
use tower::{Layer, ServiceBuilder};
use tower_governor::{governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor, GovernorLayer};

pub fn get_rate_limiter_layer(
    rate_limiter_per_second: u64, 
    rate_limiter_burst_size: u32
) -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    let governor_config = GovernorConfigBuilder::default()
        .per_second(rate_limiter_per_second)
        .burst_size(rate_limiter_burst_size)
        .finish()
        .unwrap();

    let rate_limiter = GovernorLayer {
        config: Arc::new(governor_config),
    };

    return rate_limiter;
}
