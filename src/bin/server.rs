use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    info!("Server starting...")
}