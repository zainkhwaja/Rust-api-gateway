use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};
use crate::state::SharedState;

pub async fn cache_middleware(State(_state): State<SharedState>, req: Request<Body>, next: Next) -> impl IntoResponse {
    // Redis response caching can be implemented here.
    next.run(req).await
}
