use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use axum::{routing::get, Router};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use tokio::{net::TcpListener, runtime};

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

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

async fn say_hello() -> &'static str {
    "Heya!"
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
