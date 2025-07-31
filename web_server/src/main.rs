use axum::routing::post;
use axum::{middleware, routing::get, Router};
use common::configuration::{self, ServiceConfiguration};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::{net::TcpListener, runtime};
use tower_http::cors::CorsLayer;
use web_server::metrics::{monitor_endpoint, serve_metrics, Metrics};
use web_server::{scraper, web_api, AppState};

fn main() {
    let config = load_configuration();

    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::from_str(&config.log_level).unwrap())
        .init()
        .unwrap();

    info!("Using configuration: {:?}", config);

    let app_metrics = Metrics::default();

    let tokio_runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("Runtime was expected to be created.");

    tokio_runtime.block_on(async move {
        //TODO: refactor this
        let db_user = config.db_user.clone().unwrap();
        let db_password = config.db_password.clone().unwrap();
        let db_host = config.db_host.clone().unwrap();
        let connection_string = format!("postgres://{}:{}@{}", db_user, db_password, db_host);
        let pg_pool = PgPoolOptions::new().connect(connection_string.as_str()).await.unwrap();

        let state = AppState {
            ping_msg: "The state of ping.".to_string(),
            categories: config.categories,
            metrics: Arc::new(RwLock::new(app_metrics)),
            pg_pool: Arc::new(pg_pool),
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

fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/api/ping", get(web_api::ping))
        .route("/api/incidents/count", get(web_api::count_incidents))
        .route("/api/incidents/all", get(web_api::get_all_incidents))
        .route("/scraper", post(scraper::scraper_api::submit_rss))
        .route("/metrics", get(serve_metrics))
        .route_layer(middleware::from_fn_with_state(state.clone(), monitor_endpoint))
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
