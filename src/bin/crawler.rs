extern crate core;

use enel_stop::start_crawler_service;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::env;

/**
URL: https://www.e-distributie.com/content/dam/e-distributie/outages/rss/enel_rss_muntenia.xml
 */
fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let cli_arg = env::args().nth(1);
    let file_path = match cli_arg {
        Some(file_path) => {
            let file_exists = std::path::Path::new(&file_path).exists();

            if !file_exists {
                panic!("Configuration file does not exist!")
            }
            file_path
        }
        None => panic!("Configuration file has not been provided."),
    };

    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();

    start_crawler_service(&file_path, &redis_client);
}
