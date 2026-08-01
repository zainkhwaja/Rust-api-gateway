use std::path::Path;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
    pub rate_limit: RateLimitConfig,
    pub cache: CacheConfig,
    pub routes: Vec<RouteConfig>,
    pub prometheus: PrometheusConfig,
    pub tracing: TracingConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub bind: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: Option<String>,
    pub api_keys: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests: u64,
    pub window_secs: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_secs: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RouteConfig {
    pub path: String,
    pub upstream: String,
    pub jwt_required: bool,
    pub api_key_required: bool,
    pub cache_ttl_secs: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PrometheusConfig {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TracingConfig {
    pub otlp_endpoint: Option<String>,
}

impl Config {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let bytes = tokio::fs::read(path_ref)
            .await
            .with_context(|| format!("Failed to read configuration file {}", path_ref.display()))?;
        let config = serde_yaml::from_slice(&bytes)
            .context("Failed to parse YAML configuration")?;
        Ok(config)
    }
}
