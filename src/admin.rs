use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Router};
use crate::{config::Config, state::SharedState};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/admin/reload", post(reload_config))
        .route("/health", get(health_handler))
}

pub async fn reload_config(State(state): State<SharedState>) -> impl IntoResponse {
    let new_config = match Config::load(&state.config_path).await {
        Ok(config) => config,
        Err(err) => {
            tracing::error!(error = ?err, "Configuration reload failed");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Reload failed").into_response();
        }
    };

    *state.config.write().await = new_config;
    (StatusCode::OK, "Configuration reloaded").into_response()
}

pub async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
