use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use warp::Filter;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    info!("Server starting...");

    let hello = warp::path("api").map(|| "Hello");

    // TODO: understand Into and From traits
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}