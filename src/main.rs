mod admin;
mod auth;
mod cache;
mod config;
mod gateway;
mod metrics;
mod rate_limit;
mod state;
mod telemetry;

use std::{net::SocketAddr, sync::Arc};
use axum::{routing::{any, get}, Router};
use hyper::Server;
use tower_http::trace::TraceLayer;
use anyhow::Context;
use config::Config;
use metrics::Metrics;
use state::AppState;
use telemetry::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::var("GATEWAY_CONFIG").unwrap_or_else(|_| "config.yaml".to_string());
    init_tracing();

    let config = Config::load(&config_path).await.context("Failed to load config")?;
    let redis_client = redis::Client::open(config.redis.url.clone())?;
    let metrics = Metrics::new();

    let state = Arc::new(AppState {
        config: Arc::new(tokio::sync::RwLock::new(config)),
        config_path,
        redis_client,
        metrics,
    });

    let app = Router::new()
        .route("/metrics", get(metrics::metrics_handler))
        .merge(admin::router())
        .merge(
            Router::new()
                .fallback(any(gateway::proxy_handler))
                .layer(axum::middleware::from_fn(auth::auth_middleware))
                .layer(axum::middleware::from_fn(rate_limit::rate_limit_middleware))
                .layer(axum::middleware::from_fn(cache::cache_middleware)),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!(%addr, "starting Rust API gateway");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("server terminated unexpectedly")?;

    Ok(())
}
