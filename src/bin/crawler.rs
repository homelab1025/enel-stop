extern crate core;

use config::{Config, FileFormat};
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;
use std::{env, error::Error};

use enel_stop::{configuration::ServiceConfiguration, start_crawler_service};

fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
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

    let raw_config = Config::builder()
        .add_source(config::File::new(&file_path, FileFormat::Toml))
        .add_source(config::Environment::default().separator("__"))
        .build()
        .unwrap();

    debug!("---- Environment variables ----");
    for env_var in std::env::vars() {
        debug!("{} = {}", env_var.0, env_var.1)
    }

    let config_result: Result<ServiceConfiguration, Box<dyn Error>> =
        ServiceConfiguration::new(&raw_config);

    match config_result {
        Ok(service_config) => {
            let mut redis_url = String::from("redis://");
            redis_url.push_str(&service_config.redis_server);
            redis_url.push('/');

            let redis_client = if service_config.store_enabled {
                Some(redis::Client::open(redis_url).unwrap())
            } else {
                None
            };

            start_crawler_service(&service_config, redis_client.as_ref());
        }
        Err(err) => {
            panic!("There was an error when loading the configuration: {}", err);
        }
    }
}
