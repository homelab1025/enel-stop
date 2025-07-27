use crate::metrics::Metrics;
use redis::aio::ConnectionLike;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub mod metrics;
pub mod scraper;
pub mod web_api;

#[derive(Clone)]
pub struct AppState<T>
where
    T: ConnectionLike + Send + Sync,
{
    pub ping_msg: String,
    pub redis_conn: Arc<Mutex<T>>,
    pub pg_pool: Arc<Pool<Postgres>>,
    pub categories: Vec<String>,
    pub metrics: Arc<RwLock<Metrics>>,
}
