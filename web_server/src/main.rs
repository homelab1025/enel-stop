use axum::routing::post;
use axum::{Router, routing::get};
use common::configuration::{self, ServiceConfiguration};
use log::{LevelFilter, info};
use prometheus_client::registry::Registry;
use redis::aio::MultiplexedConnection;
use simple_logger::SimpleLogger;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::{net::TcpListener, runtime};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use web_server::metrics::Metrics;
use web_server::{AppState, scraper, web_api};

fn main() {
    let config = load_configuration();

    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::from_str(&config.log_level).unwrap())
        .init()
        .unwrap();

    info!("Using configuration: {:?}", config);

    let redis_string = config.redis_server.expect("Redis server must be configured.");
    let client = redis::Client::open(redis_string).expect(
        "Redis client could not be created. Check connection string or remove it if you don't want to store results.",
    );

    let mut metrics_registry = Registry::with_prefix_and_labels("enel", [].into_iter());
    let app_metrics = Metrics::new(&mut metrics_registry);

    // let gauge_full: Gauge = Gauge::default();
    // let incidents_count: Counter = Counter::default();
    // let failures_count: Counter = Counter::default();

    // let _ = register_metrics(&app_metrics);

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
            categories: config.categories,
            metrics: Arc::new(RwLock::new(app_metrics)),
        };

        let mut app = create_app(state);

        if config.cors_permissive {
            app = app.layer(CorsLayer::permissive());
        }
        let addr = format!("0.0.0.0:{}", config.http_port);
        let listener = TcpListener::bind(addr).await.expect("Could not open port.");

        axum::serve(listener, app).await.unwrap();
    });
}

fn create_app(state: AppState<MultiplexedConnection>) -> Router {
    Router::new()
        .route("/api/ping", get(web_api::ping))
        .route("/api/incidents/count", get(web_api::count_incidents))
        .route("/api/incidents/all", get(web_api::get_all_incidents))
        .route("/scraper", post(scraper::scraper_api::submit_rss))
        .fallback_service(ServeDir::new("web_assets"))
        .with_state(state)
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
