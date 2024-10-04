use regex::Regex;
use std::{ops::Add, thread, time::Duration};

use log::{debug, info};

use redis::{Commands, RedisError};

use crate::configuration::ServiceConfiguration;

pub mod configuration;
mod rss_reader;

pub fn start_crawler_service(config: &ServiceConfiguration, redis_client: &redis::Client) {
    info!("Using configuration: {}", config);

    let redis_connection = redis_client.get_connection();
    let location_extract_pattern = r"(.*?) Judet: (\w+)\s+Localitate: (.+)";
    let location_extractor = Regex::new(location_extract_pattern).unwrap();

    match redis_connection {
        Ok(mut conn) => {
            info!("Redis connection established.");

            let parse_func = || {
                debug!("running the parser");
                let items = rss_reader::parse_rss(&config.url, &config.categories);

                items.iter().for_each(|item| {
                    if let Some(title) = item.title.as_ref() {
                        let pub_date = item.pub_date.as_ref().unwrap();
                        let id = item.guid.as_ref().unwrap();

                        debug!(
                            "Found: {} published at {} with GUID {}",
                            title,
                            pub_date,
                            id.value()
                        );

                        if let Some(captures) = location_extractor.captures(title) {
                            let interval = captures.get(1).unwrap().as_str();
                            let judet = captures.get(2).unwrap().as_str();
                            let localitate = captures.get(3).unwrap().as_str();

                            let location = String::from(judet).add("-").add(localitate);
                            // info!("Creating a key from {} and {}: {}", judet, localitate, key);
                            let _: Result<String, RedisError> = conn.set(location, interval.trim());
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

fn create_loop<F: FnMut()>(mut processor: F, refresh_ms: u64) {
    info!("Starting to query every {} seconds", refresh_ms);

    loop {
        processor();
        thread::sleep(Duration::from_millis(refresh_ms));
    }
}
