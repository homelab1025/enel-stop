use std::{thread, time::Duration};

use log::{debug, info};

use redis::{Commands, RedisError};
use regex::Regex;

use crate::configuration::ServiceConfiguration;
use serde::Serialize;

pub mod configuration;
mod rss_reader;

#[derive(Debug, Serialize)]
struct Record {
    id: String,
    judet: String,
    localitate: String,
    title: String,
    description: String,
}

pub fn start_crawler_service(config: &ServiceConfiguration, redis_client: Option<&redis::Client>) {
    info!("Using configuration: {}", config);

    let mut redis_conn = redis_client.map(|client| client.get_connection().unwrap());

    let location_extract_pattern = r"(.*?) Judet: (\w+)\s+Localitate: (.+)";
    let location_extractor = Regex::new(location_extract_pattern).unwrap();

    info!("Redis connection established.");
    info!("Starting to query every {} seconds", config.refresh_ms);

    loop {
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
                    // let interval = captures.get(1).unwrap().as_str();
                    let judet = captures.get(2).unwrap().as_str();
                    let localitate = captures.get(3).unwrap().as_str();

                    let r = Record {
                        id: id.value.to_string(),
                        judet: judet.to_string(),
                        localitate: localitate.to_string(),
                        title: title.to_string(),
                        description: item.description.as_ref().unwrap().to_string(),
                    };

                    // info!("Creating a key from {} and {}: {}", judet, localitate, key);
                    let ser = serde_json::to_string(&r).unwrap();
                    info!("Adding record: {}", ser);

                    if let Some(ref mut conn) = redis_conn.as_mut() {
                        let _: Result<String, RedisError> = conn.set(id.value(), ser);
                    }
                }
            };
        });

        thread::sleep(Duration::from_millis(config.refresh_ms));
    }
}
