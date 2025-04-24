use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
};

use axum::{
    http::{self, StatusCode},
    routing::get,
    Router,
};
use common::configuration::{self, ServiceConfiguration};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::{net::TcpListener, runtime};
use web_server::call_migration;
use web_server::migration::sorted_set::{MigrationProcess, SortedSetMigration};

pub mod migration;
fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let config = load_configuration();

    let redis_string = config.redis_server.expect("Redis server must be configured.");
    let client = redis::Client::open(redis_string).expect(
        "Redis client could not be created. Check connection string or remove it if you don't want to store results.",
    );

    let mut redis_conn = client.get_connection().expect("Could not connect to redis.");

    let mut sorted_set_migration = SortedSetMigration {};
    let mut migrations: Vec<&mut dyn MigrationProcess> = vec![&mut sorted_set_migration];
    call_migration(&mut migrations, &mut redis_conn);

    let core_count = get_core_count().expect("Could not detect number of cores");
    info!("Detected {} cores.", core_count);

    let rt = runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .expect("Runtime was expected to be created.");

    rt.block_on(async move {
        let app = Router::new().route("/", get(say_hello));
        let listener = TcpListener::bind("0.0.0.0:9090").await.expect("Could not open port.");

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

async fn say_hello() -> (http::StatusCode, &'static str) {
    (StatusCode::OK, "Heya!")
}

fn get_core_count() -> Result<u8, Error> {
    let cpuinfo_file = File::open("/proc/cpuinfo")?;

    let mut count: u8 = 0;

    let reader = BufReader::new(cpuinfo_file);
    for line in reader.lines() {
        if line?.starts_with("processor\t:") {
            count += 1;
        }
    }

    Ok(count)
}
