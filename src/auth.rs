use axum::{body::Body, extract::State, http::{Request, StatusCode}, middleware::Next, response::IntoResponse};
use crate::{config::AuthConfig, state::SharedState};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;

pub async fn auth_middleware(State(state): State<SharedState>, req: Request<Body>, next: Next) -> impl IntoResponse {
    let config = state.config.read().await;
    let auth = &config.auth;

    if auth.jwt_secret.is_none() && auth.api_keys.is_empty() {
        return next.run(req).await;
    }

    let headers = req.headers();
    if let Some(token_header) = headers.get(axum::http::header::AUTHORIZATION) {
        if let Ok(token_str) = token_header.to_str() {
            let token = token_str.trim_start_matches("Bearer ").trim();
            if validate_jwt(token, auth).is_ok() {
                return next.run(req).await;
            }
        }
    }

    if let Some(api_key_header) = headers.get("x-api-key") {
        if let Ok(api_key) = api_key_header.to_str() {
            if auth.api_keys.iter().any(|key| key == api_key) {
                return next.run(req).await;
            }
        }
    }

    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}

fn validate_jwt(token: &str, auth: &AuthConfig) -> Result<(), jsonwebtoken::errors::Error> {
    let secret = auth.jwt_secret.as_ref().ok_or_else(|| jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))?;
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();
    let _token_data = decode::<Value>(token, &decoding_key, &validation)?;
    Ok(())
}
