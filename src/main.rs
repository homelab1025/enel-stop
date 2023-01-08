extern crate core;

use std::{env, thread};
use std::time::Duration;

use config::{Config, FileFormat};
use log::{debug, info, LevelFilter};
use rss::{Category, Channel, Item};
use simple_logger::SimpleLogger;

use crate::configuration::ServiceConfiguration;

mod configuration;

/**
URL: https://www.e-distributie.com/content/dam/e-distributie/outages/rss/enel_rss_muntenia.xml
 */
fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .init().unwrap();

    let cli_arg = env::args().nth(1);
    let file_path = validate_config_file(cli_arg);

    let config = Config::builder()
        .add_source(config::File::new(&file_path, FileFormat::Toml))
        .add_source(config::Environment::default())
        .build().unwrap();

    for v in std::env::vars() {
        println!("{} = {}", v.0, v.1)
    }

    let config_result = ServiceConfiguration::new(&config);

    match config_result {
        Err(err) => {
            panic!("There was an error when loading the configuration: {}", err)
        }
        Ok(config) => {
            info!("Using configuration: {}", config);
            let filtering_categs = convert_config_categs(&config.categories);

            loop {
                print_incidents(&config, &filtering_categs);
                thread::sleep(Duration::from_millis(config.refresh_ms));
            }
        }
    }
}

fn print_incidents(config: &ServiceConfiguration, filtering_categs: &Vec<Category>) {
    let content = reqwest::blocking::get(&config.url)
        .unwrap()
        .bytes()
        .unwrap();

    let channel = Channel::read_from(&content[..]).unwrap();

    debug!("Scheduled downtime locations: {}", channel.items.len());

    let title_filtered: Vec<&Item> = channel.items.iter().filter(|x| {
        match x.title.as_ref() {
            Some(title) => title.to_lowercase().contains("ilfov"),
            None => false
        }
    }).collect();

    let category_filtered: Vec<&Item> = channel.items.iter().filter(|item| {
        filtering_categs.iter().all(|x| item.categories.contains(x))
    }).collect();

    if title_filtered.len() > 1 {
        debug!("Found: {}", title_filtered.len());
        // send_sms(&title_filtered);
        info!("Found: {}", category_filtered.len());
        send_sms(&category_filtered);
    }
}

fn convert_config_categs(config_categs: &Vec<String>) -> Vec<Category> {
    config_categs.iter().map(|x| {
        Category {
            domain: None,
            name: String::from(x),
        }
    }).collect()
}

fn validate_config_file(cli_arg: Option<String>) -> String {
    match cli_arg {
        Some(file_path) => {
            let file_exists = std::path::Path::new(&file_path).exists();

            match file_exists {
                true => {
                    println!("Using file: {}", &file_path);
                    file_path
                }
                false => {
                    panic!("Configuration file does not exist!")
                }
            }
        }
        None => panic!("Configuration file has not been provided.")
    }
}

fn send_sms(locations_counter: &Vec<&Item>) {
    for x in locations_counter {
        info!("Location: {}", x.title.as_ref().unwrap());
        // println!("Item: {}", x);
    }
}