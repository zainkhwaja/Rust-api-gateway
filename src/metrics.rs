use axum::{extract::State, http::{header::CONTENT_TYPE, StatusCode}, response::IntoResponse};
use prometheus::{Encoder, HistogramVec, IntCounterVec, Opts, Registry, TextEncoder};
use crate::state::SharedState;

#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,
    pub http_requests_total: IntCounterVec,
    pub request_duration_seconds: HistogramVec,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let http_requests_total = IntCounterVec::new(
            Opts::new("http_requests_total", "Total number of HTTP requests."),
            &["method", "path", "status"],
        )
        .expect("metrics registration failed");

        let request_duration_seconds = HistogramVec::new(
            prometheus::HistogramOpts::new(
                "http_request_duration_seconds",
                "Duration of HTTP requests in seconds.",
            ),
            &["method", "path"],
        )
        .expect("metrics registration failed");

        registry.register(Box::new(http_requests_total.clone())).ok();
        registry.register(Box::new(request_duration_seconds.clone())).ok();

        Metrics {
            registry,
            http_requests_total,
            request_duration_seconds,
        }
    }
}

pub async fn metrics_handler(State(state): State<SharedState>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();
    let mut buffer = Vec::new();
    if let Err(err) = encoder.encode(&metric_families, &mut buffer) {
        tracing::error!(error = %err, "Failed to encode Prometheus metrics");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Metrics encoding failed");
    }

    (
        StatusCode::OK,
        [(CONTENT_TYPE, encoder.format_type())],
        String::from_utf8(buffer).unwrap_or_default(),
    )
}
