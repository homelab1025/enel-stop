extern crate core;

use std::env;

use rss::Channel;

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

            let affected_locations = channel.items.len();
            println!("Scheduled downtime locations: {}", affected_locations);

            if affected_locations > 10 {
                send_sms(affected_locations);
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

fn send_sms(locations_counter: usize) {
    todo!()
}