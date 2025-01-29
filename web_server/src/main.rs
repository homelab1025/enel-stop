use std::env;

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let cli_arg = env::args().nth(1).unwrap_or(String::from("8080"));
    let _port = cli_arg.parse::<u16>().unwrap();

    info!("Server starting...");
}
