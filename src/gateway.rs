use axum::{extract::State, body::Body, http::{Error, Request, StatusCode, Uri}, response::IntoResponse};
use hyper::Client;
use crate::state::SharedState;

pub async fn proxy_handler(State(state): State<SharedState>, mut req: Request<Body>) -> impl IntoResponse {
    let config = state.config.read().await;
    let route = match config.routes.first() {
        Some(route) => route,
        None => return (StatusCode::NOT_FOUND, "No upstream routes configured").into_response(),
    };

    let upstream_uri = match build_target_uri(&route.upstream, req.uri()) {
        Ok(uri) => uri,
        Err(err) => {
            tracing::error!(error = ?err, "Failed to build upstream URI");
            return (StatusCode::BAD_GATEWAY, "Invalid upstream URI").into_response();
        }
    };

    *req.uri_mut() = upstream_uri;
    let client = Client::new();
    match client.request(req).await {
        Ok(response) => response.into_response(),
        Err(err) => {
            tracing::error!(error = ?err, "Upstream proxy failed");
            (StatusCode::BAD_GATEWAY, "Upstream request failed").into_response()
        }
    }
}

fn build_target_uri(base: &str, original: &Uri) -> Result<Uri, Error> {
    let base_uri: Uri = base.parse()?;
    let scheme = base_uri.scheme_str().unwrap_or("http");
    let authority = base_uri.authority().map(|a| a.as_str()).unwrap_or_default();
    let path_and_query = original.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");

    format!("{}://{}{}", scheme, authority, path_and_query).parse()
}
