use axum::extract::State;
use axum::{
    http::{self, StatusCode},
    routing::get,
    Router,
};
use common::configuration::{self, ServiceConfiguration};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use std::env;
use std::ops::Deref;
use std::sync::Arc;
use tokio::{net::TcpListener, runtime};
use web_server::migration::sorted_set::SortedSetMigration;
use web_server::migration::MigrationProcess;

pub mod migration;

#[derive(Clone)]
struct AppState {
    ping_msg: String,
}

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let config = load_configuration();

    let redis_string = config.redis_server.expect("Redis server must be configured.");
    let client = redis::Client::open(redis_string).expect(
        "Redis client could not be created. Check connection string or remove it if you don't want to store results.",
    );

    let mut redis_conn = client.get_connection().expect("Could not connect to redis.");

    let mut sorted_set_migration = SortedSetMigration::default();
    let mut migrations: Vec<&mut dyn MigrationProcess> = vec![&mut sorted_set_migration];
    // call_migration(&mut migrations, &mut redis_conn);

    let tokio_runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("Runtime was expected to be created.");

    tokio_runtime.block_on(async move {
        let state = Arc::new(AppState {
            ping_msg: "The state of ping.".to_string(),
        });

        let app = Router::new()
            .route("/ping", get(say_hello))
            .route("/incidents/count", get(count_incidents))
            .with_state(state);

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

async fn say_hello(State(state): State<Arc<AppState>>) -> (StatusCode, String) {
    let a = state.as_ref().ping_msg.deref();
    let response = format!("Hello {}!", a);
    (StatusCode::OK, response)
}

async fn count_incidents(State(state): State<Arc<AppState>>) -> (StatusCode, String) {
    (StatusCode::OK, "Incidents".to_string())
}
