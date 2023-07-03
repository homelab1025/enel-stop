use std::{error::Error, thread, time::Duration};
use std::io::Read;

use config::{Config, FileFormat};
use log::{debug, error, info};
use reqwest::blocking::Client;
use rss::{Category, Channel, Item};

use crate::{configuration::ServiceConfiguration};

mod configuration;
mod notifications;

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
                parse_rss(&config.url, &convert_config_categs(&config.categories), &rss_client);
            };

            create_loop(parse_func, config.refresh_ms);
        }
        Err(err) => {
            panic!("There was an error when loading the configuration: {}", err)
        }
    }
}

fn parse_rss(url: &str, filter_categs: &Vec<Category>, rss_client: &Client) {
    info!("Filtering for categs: {:?}", filter_categs);

    let channel_resp = rss_client.get(url).send();

    match channel_resp {
        Ok(mut resp) => {
            let mut buffer = Vec::new();
            let read_result = resp.read_to_end(&mut buffer);
            if read_result.is_err() {
                return;
            }

            let channel = match Channel::read_from(&buffer[..]) {
                Ok(channel) => channel,
                Err(err) => {
                    error!("There was an error parsing the RSS: {}", err);
                    return;
                }
            };

            let filtered_items = filter_incidents(&channel.items, filter_categs);

            filtered_items.iter().for_each(|item| {
                if let Some(title) = item.title.as_ref() {
                    info!("Found: {}", title);
                };
            })
        }
        Err(err) => {
            error!("There was an error making the request for the RSS: {}", err);
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

fn filter_incidents(all_incidents: &[Item], filtering_categs: &[Category]) -> Vec<Item> {
    all_incidents
        .iter()
        .filter(|item| filtering_categs.iter().all(|x| item.categories.contains(x)))
        .cloned()
        .collect()
}

/// Convert the categories from the configuration into RSS categories.
fn convert_config_categs(config_categs: &[String]) -> Vec<Category> {
    config_categs
        .iter()
        .map(|x| Category {
            domain: None,
            name: String::from(x),
        })
        .collect()
}

#[cfg(test)]
mod lib_tests {
    use rss::{Category, ItemBuilder};

    use crate::{convert_config_categs, filter_incidents};

    #[test]
    fn convert_config_categs_works() {
        let config_categs = ["one".to_string(), "two".to_string()];
        let expected = vec![
            Category {
                domain: None,
                name: "one".to_string(),
            },
            Category {
                domain: None,
                name: "two".to_string(),
            },
        ];
        let result = convert_config_categs(&config_categs);

        assert_eq!(expected, result);
    }

    #[test]
    fn convert_config_categs_empty() {
        let config_categs = [];
        let expected: Vec<Category> = vec![];
        let result = convert_config_categs(&config_categs);

        assert_eq!(expected, result);
    }

    #[test]
    fn filter_incidents_correctly() {
        const FILTER_CATEG_1: &str = "one";
        const FILTER_CATEG_2: &str = "two";

        let config_categs = [FILTER_CATEG_1.to_string(), FILTER_CATEG_2.to_string()];
        let filtering_categs = convert_config_categs(&config_categs);

        let incorrect_cats = vec![
            Category {
                domain: None,
                name: "c1".to_string(),
            },
            Category {
                domain: None,
                name: "c2".to_string(),
            },
        ];

        let partial_correct_cats = vec![
            Category {
                domain: None,
                name: FILTER_CATEG_1.to_string(),
            },
            Category {
                domain: None,
                name: "boom".to_string(),
            },
        ];

        let single_cats = vec![Category {
            domain: None,
            name: FILTER_CATEG_1.to_string(),
        }];

        let correct_cats = vec![
            Category {
                domain: None,
                name: FILTER_CATEG_1.to_string(),
            },
            Category {
                domain: None,
                name: FILTER_CATEG_2.to_string(),
            },
        ];

        let all_incidents = [
            ItemBuilder::default()
                .categories(incorrect_cats)
                .title(Some("incorrect".to_string()))
                .build(),
            ItemBuilder::default()
                .categories(partial_correct_cats)
                .title(Some("partial_correct".to_string()))
                .build(),
            ItemBuilder::default()
                .categories(single_cats)
                .title(Some("single_category".to_string()))
                .build(),
            ItemBuilder::default()
                .categories(correct_cats)
                .title(Some(String::from("correct")))
                .build(),
        ];

        let result = filter_incidents(&all_incidents, &filtering_categs);

        assert_eq!(1, result.len());
        assert_eq!("correct", result.get(0).unwrap().title.as_ref().unwrap());
    }
}
