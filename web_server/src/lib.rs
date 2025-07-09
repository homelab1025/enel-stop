use crate::metrics::Metrics;
use redis::aio::ConnectionLike;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub mod metrics;
pub mod scraper;
pub mod web_api;

#[derive(Clone, Default)]
pub struct AppState<T>
where
    T: ConnectionLike + Send + Sync,
{
    pub ping_msg: String,
    pub redis_conn: Arc<Mutex<T>>,
    pub categories: Vec<String>,
    pub metrics: Arc<RwLock<Metrics>>,
}
