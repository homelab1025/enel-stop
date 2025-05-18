use axum::{routing::get, Router};
use common::configuration::{self, ServiceConfiguration};
use log::{info, LevelFilter};
use redis::aio::{ConnectionLike};
use simple_logger::SimpleLogger;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{net::TcpListener, runtime};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub mod migration;

#[derive(Clone)]
struct AppState<T>
where
    T: ConnectionLike + Send + Sync,
{
    ping_msg: String,
    redis_conn: Arc<Mutex<T>>,
}

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let config = load_configuration();

    let redis_string = config.redis_server.expect("Redis server must be configured.");
    let client = redis::Client::open(redis_string).expect(
        "Redis client could not be created. Check connection string or remove it if you don't want to store results.",
    );

    // let mut redis_conn = client.get_connection().expect("Could not connect to redis.");

    // let mut sorted_set_migration = SortedSetMigration::default();
    // let mut migrations: Vec<&mut dyn MigrationProcess> = vec![&mut sorted_set_migration];
    // call_migration(&mut migrations, &mut redis_conn);

    let tokio_runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("Runtime was expected to be created.");

    tokio_runtime.block_on(async move {
        let async_redis_conn = client
            .get_multiplexed_async_connection()
            .await
            .expect("Could not ASYNC connect to Redis.");

        let state = AppState {
            ping_msg: "The state of ping.".to_string(),
            redis_conn: Arc::new(Mutex::new(async_redis_conn)),
        };

        let mut app = Router::new()
            .route("/api/ping", get(api::ping))
            .route("/api/incidents/count", get(api::count_incidents))
            .fallback_service(ServeDir::new("web_assets"))
            .with_state(state);

        if config.cors_permissive {
            app = app.layer(CorsLayer::permissive());
        }
        let addr = format!("0.0.0.0:{}", config.http_port);
        let listener = TcpListener::bind(addr).await.expect("Could not open port.");

        axum::serve(listener, app).await.unwrap();
    });
}

mod api {
    use crate::AppState;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::Json;
    use redis::aio::ConnectionLike;
    use redis::RedisError;
    use serde::Serialize;
    use std::ops::Deref;

    #[derive(Debug, Serialize, Clone)]
    pub struct RecordCount {
        count: u64,
    }

    #[derive(Debug, Serialize, Clone)]
    pub struct Ping {
        ping: String,
    }

    pub async fn count_incidents<T>(state: State<AppState<T>>) -> Result<Json<RecordCount>, (StatusCode, String)>
    where
        T: ConnectionLike + Send + Sync,
    {
        let mut conn_guard = state.redis_conn.lock().await;
        let conn = &mut *conn_guard;
        let counter: Result<u64, RedisError> = redis::cmd("DBSIZE").query_async(conn).await;

        match counter {
            Ok(key_count) => Ok(Json(RecordCount { count: key_count })),
            Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
        }
    }

    pub async fn ping<T>(state: State<AppState<T>>) -> Result<Json<Ping>, (StatusCode, String)>
    where
        T: ConnectionLike + Send + Sync,
    {
        let a = state.ping_msg.deref();
        let response = format!("Hello {}!", a);
        Ok(Json(Ping { ping: response }))
    }
}

fn load_configuration() -> ServiceConfiguration {
    let cli_arg = env::args().nth(1);
    let config = cli_arg.map(|file_path| configuration::get_configuration(&file_path));

    if config.is_none() {
        panic!("No configuration has been provided.");
    }

    let config = config.unwrap();
    if config.is_err() {
        panic!("some other config issue: {}", config.unwrap_err());
    }
    let config = config.unwrap();

    info!("Configuration: {}", config);

    config
}
