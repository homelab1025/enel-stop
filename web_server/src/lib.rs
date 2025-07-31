use crate::metrics::Metrics;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod metrics;
pub mod scraper;
pub mod web_api;

#[derive(Clone)]
pub struct AppState {
    pub ping_msg: String,
    pub pg_pool: Arc<Pool<Postgres>>,
    pub categories: Vec<String>,
    pub metrics: Arc<RwLock<Metrics>>,
}
