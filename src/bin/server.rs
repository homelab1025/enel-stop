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

    let api = warp::path("api");

    let ping_api = api.and(warp::path("ping")).map(|| "pong");
    let store_api = api.and(warp::path("store")).map(|| "stire");

    let all_routes = ping_api.or(store_api);

    // TODO: understand Into and From traits
    warp::serve(all_routes).run(([127, 0, 0, 1], 3030)).await;
}