extern crate core;

use std::env;

use rss::{Category, Channel, Item};

mod configuration;

/**
URL: https://www.e-distributie.com/content/dam/e-distributie/outages/rss/enel_rss_muntenia.xml
 */
fn main() {
    let cli_arg = env::args().nth(1);
    let file_path = validate_config_file(cli_arg);

    let config_result = configuration::ServiceConfiguration::new(&file_path);

    match config_result {
        Err(err) => {
            panic!("There was an error when loading the configuration: {}", err)
        }
        Ok(config) => {
            println!("Using configuration: {}", config);

            let content = reqwest::blocking::get(config.url)
                .unwrap()
                .bytes()
                .unwrap();

            let channel = Channel::read_from(&content[..]).unwrap();

            println!("Scheduled downtime locations: {}", channel.items.len());

            let title_filtered: Vec<&Item> = channel.items.iter().filter(|x| {
                match x.title.as_ref() {
                    Some(title) => title.to_lowercase().contains("ilfov"),
                    None => false
                }
            }).collect();

            let category_filtered: Vec<&Item> = channel.items.iter().filter(|x| {
                x.categories.contains(&Category {
                    domain: None,
                    name: String::from("Jud. ILFOV"),
                })
            }).collect();

            if title_filtered.len() > 1 {
                println!("Found: {}", title_filtered.len());
                send_sms(&title_filtered);
                println!("-----------------------------");
                println!("Found: {}", category_filtered.len());
                send_sms(&category_filtered);
            }
        }
    }
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
        println!("Location: {}", x.title.as_ref().unwrap());
        // println!("Item: {}", x);
    }
}