use axum::{body::Body, extract::State, http::{Request, StatusCode}, middleware::Next, response::IntoResponse};
use redis::AsyncCommands;
use crate::state::SharedState;

pub async fn rate_limit_middleware(State(state): State<SharedState>, req: Request<Body>, next: Next) -> impl IntoResponse {
    let config = state.config.read().await;
    if !config.rate_limit.enabled {
        return next.run(req).await;
    }

    let key = format!("rate_limit:global:{}", req.uri().path());
    let mut conn = match state.redis_client.get_tokio_connection().await {
        Ok(conn) => conn,
        Err(err) => {
            tracing::warn!(error = %err, "Redis unavailable for rate limiting");
            return next.run(req).await;
        }
    };

    let current: u64 = match conn.incr(&key, 1u64).await {
        Ok(value) => value,
        Err(err) => {
            tracing::warn!(error = %err, "Rate limit increment failed");
            return next.run(req).await;
        }
    };

    if current == 1 {
        let _ = conn.expire(&key, config.rate_limit.window_secs as i64).await;
    }

    if current > config.rate_limit.requests {
        return (StatusCode::TOO_MANY_REQUESTS, "Too many requests").into_response();
    }

    next.run(req).await
}
