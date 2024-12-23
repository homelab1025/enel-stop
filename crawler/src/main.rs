use config::{Config, FileFormat};
use log::{debug, error, info, LevelFilter};
use redis::RedisError;
use regex::Regex;
use reqwest::{
    blocking::Client,
    cookie::Jar,
    header::{self, HeaderValue},
};
use simple_logger::SimpleLogger;
use std::{env, error::Error, thread, time::Duration};

// use enel_stop::{configuration::ServiceConfiguration, start_crawler_service};
mod configuration;
use configuration::ServiceConfiguration;

mod rss_reader;

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

pub fn start_crawler_service(config: &ServiceConfiguration, redis_client: Option<&redis::Client>) {
    info!("Using configuration: {}", config);

    let mut redis_conn = redis_client.map(|client| client.get_connection().unwrap());

    if let Some(ref _conn) = redis_conn {
        info!("Redis connection established.");
    }

    let location_extract_pattern = r"(.*?) Judet: (\w+)\s+Localitate: (.+)";
    let location_extractor = Regex::new(location_extract_pattern).unwrap();

    info!("Starting to query every {} seconds", config.refresh_ms);

    let headers = chrome_headers();
    let cookie_store = std::sync::Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_provider(cookie_store)
        .tls_info(true)
        .use_rustls_tls()
        .connection_verbose(true)
        .default_headers(headers)
        .build();

    match client {
        Err(err) => {
            panic!("Can not instantiate RSS client: {}", err);
        }

        Ok(rss_client) => {
            loop {
                debug!("running the parser");
                let items = rss_reader::parse_rss(&config.url, &config.categories, &rss_client);

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

                            let r = common::Record {
                                id: id.value.to_string(),
                                judet: judet.to_string(),
                                localitate: localitate.to_string(),
                                title: title.to_string(),
                                description: item.description.as_ref().unwrap().to_string(),
                            };

                            // info!("Creating a key from {} and {}: {}", judet, localitate, key);
                            let ser = serde_json::to_string(&r).unwrap();
                            info!("Adding record: {} with key {}", ser, id.value());

                            if let Some(ref mut conn) = redis_conn.as_mut() {
                                let result: Result<String, RedisError> = conn.set(id.value(), ser);
                                if result.is_err() {
                                    error!("Error saving {}", id.value());
                                }
                            }
                        }
                    };
                });

                thread::sleep(Duration::from_millis(config.refresh_ms));
            }
        }
    }
}

fn chrome_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36"));
    // headers.insert(header::ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    // headers.insert(
    //     header::ACCEPT_LANGUAGE,
    //     HeaderValue::from_static("en-US,en;q=0.9,ro;q=0.8"),
    // );
    // headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    // headers.insert(header::DNT, HeaderValue::from_static("1"));
    // headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
    // headers.insert("priority", HeaderValue::from_static("u=0, i"));
    // headers.insert(
    //     "sec-ch-ua",
    //     HeaderValue::from_static(
    //         r#""Google Chrome";v="129", "Not=A?Brand";v="8", "Chromium";v="129""#,
    //     ),
    // );
    headers
}
