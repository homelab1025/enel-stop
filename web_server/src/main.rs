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
use common::{
    configuration::{self, ServiceConfiguration},
    Record,
};
use log::{info, LevelFilter};
use redis::cmd;
use simple_logger::SimpleLogger;
use tokio::{net::TcpListener, runtime};

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let config = load_configuration();

    let redis_string = config.redis_server.expect("Redis server must be configured.");
    let client = redis::Client::open(redis_string).expect(
        "Redis client could not be created. Check connection string or remove it if you don't want to store results.",
    );

    let mut redis_conn = client.get_connection().expect("Could not connect to redis.");
    migrate_records(&mut redis_conn);

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

/// Blocking function for migrating the records stored in redis to another structure.
fn migrate_records(redis_conn: &mut redis::Connection) {
    let mut cursor = String::from("0");
    loop {
        let (next_cursor, keys): (String, Vec<String>) = cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg("12*")
            .arg("COUNT")
            .arg("1000")
            .query(redis_conn)
            .expect("Could not run SCAN command");

        keys.iter().for_each(|key| {
            create_timestamp_sorted_set(&key, redis_conn);
        });

        if next_cursor == "0" {
            info!("Wen thru all the keys.");
            break;
        }
        cursor = next_cursor.clone();
    }
}

fn create_timestamp_sorted_set(key: &String, redis_conn: &mut redis::Connection) {
    info!("KEY {}", key);
    let record_json: String = cmd("GET").arg(key).query(redis_conn).expect("Could not get the value.");
    let record: Record = serde_json::from_str(&record_json).expect("Could not deserialize.");

    info!(
        "Got key {} with time {} so timestamp is {}",
        record.id,
        record.date,
        record.date.and_hms(0, 0, 0).and_utc().timestamp()
    )
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
