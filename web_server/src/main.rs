use axum::{routing::get, Router};
use common::configuration::{self, ServiceConfiguration};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{net::TcpListener, runtime};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use web_server::api;
use web_server::api::AppState;

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let config = load_configuration();

    let redis_string = config.redis_server.expect("Redis server must be configured.");
    let client = redis::Client::open(redis_string).expect(
        "Redis client could not be created. Check connection string or remove it if you don't want to store results.",
    );

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
