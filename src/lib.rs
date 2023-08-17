use regex::Regex;
use std::{error::Error, thread, time::Duration, ops::Add};

use config::{Config, FileFormat};
use log::{debug, info};

use redis::{Commands, RedisError};
use reqwest::blocking::Client;

use crate::configuration::ServiceConfiguration;

mod configuration;
mod rss_reader;

pub fn start_crawler_service(file_path: &str, redis_client: &redis::Client) {
    let config = Config::builder()
        .add_source(config::File::new(file_path, FileFormat::Toml))
        .add_source(config::Environment::default())
        .build()
        .unwrap();

    debug!("---- Environment variables ----");
    for env_var in std::env::vars() {
        debug!("{} = {}", env_var.0, env_var.1)
    }

    let config_result: Result<ServiceConfiguration, Box<dyn Error>> =
        ServiceConfiguration::new(&config);

    match config_result {
        Ok(config) => {
            info!("Using configuration: {}", config);

            let rss_client = Client::new();
            let redis_connection = redis_client.get_connection();
            let key_extract_pattern = r"(.*?) Judet: (\w+)\s+Localitate: (\w+)";
            let key_extractor = Regex::new(key_extract_pattern).unwrap();

            match redis_connection {
                Ok(mut conn) => {
                    info!("Redis connection established.");

                    let parse_func = || {
                        debug!("running the parser");
                        let items =
                            rss_reader::parse_rss(&config.url, &config.categories, &rss_client);

                        items.iter().for_each(|item| {
                            if let Some(title) = item.title.as_ref() {
                                let pub_date = item.pub_date.as_ref().unwrap();
                                let id = item.guid.as_ref().unwrap();
                                info!(
                                    "Found: {} published at {} with GUID {}",
                                    title,
                                    pub_date,
                                    id.value()
                                );

                                if let Some(captures) = key_extractor.captures(title) {
                                    let interval = captures.get(1).unwrap().as_str();
                                    let judet = captures.get(2).unwrap().as_str();
                                    let localitate = captures.get(3).unwrap().as_str();

                                    info!("Creating a key from {} and {}.", judet, localitate);
                                    let key = String::from(judet).add("-").add(localitate);
                                    let _: Result<String, RedisError> =
                                        conn.set(key, interval.trim());
                                }
                            };
                        });
                    };

                    create_loop(parse_func, config.refresh_ms);
                }
                Err(err) => {
                    panic!("Could not get connection to redis: {}", err)
                }
            }
        }
        Err(err) => {
            panic!("There was an error when loading the configuration: {}", err)
        }
    }
}

fn create_loop<F: FnMut()>(mut processor: F, refresh_ms: u64) {
    info!("Starting to query every {} seconds", refresh_ms);

    loop {
        processor();
        thread::sleep(Duration::from_millis(refresh_ms));
    }
}
