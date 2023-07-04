use std::{error::Error, thread, time::Duration};

use config::{Config, FileFormat};
use log::{debug, info};
use reqwest::blocking::Client;
use rss::{Category};

use crate::configuration::ServiceConfiguration;

mod configuration;
mod notifications;
mod rss_reader;

pub fn start_service(file_path: &str) {
    let config = Config::builder()
        .add_source(config::File::new(file_path, FileFormat::Toml))
        .add_source(config::Environment::default())
        .build()
        .unwrap();

    debug!("---- Environment variables ----");
    for env_var in std::env::vars() {
        debug!("{} = {}", env_var.0, env_var.1)
    }

    let config_result: Result<ServiceConfiguration, Box<dyn Error>> = ServiceConfiguration::new(&config);

    match config_result {
        Ok(config) => {
            let rss_client = Client::new();
            let parse_func = || {
                debug!("running the parser");
                let items = rss_reader::parse_rss(&config.url, &config.categories, &rss_client);

                items.iter().for_each(|item| {
                    if let Some(title) = item.title.as_ref() {
                        info!("Found: {}", title);
                    };
                });
            };

            create_loop(parse_func, config.refresh_ms);
        }
        Err(err) => {
            panic!("There was an error when loading the configuration: {}", err)
        }
    }
}

fn create_loop<F: Fn()>(processor: F, refresh_ms: u64) {
    info!("Starting to query every {} seconds", refresh_ms);

    loop {
        processor();
        thread::sleep(Duration::from_millis(refresh_ms));
    }
}