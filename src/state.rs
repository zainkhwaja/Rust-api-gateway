use std::sync::Arc;
use tokio::sync::RwLock;
use crate::{config::Config, metrics::Metrics};

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub config: Arc<RwLock<Config>>,
    pub config_path: String,
    pub redis_client: redis::Client,
    pub metrics: Metrics,
}
