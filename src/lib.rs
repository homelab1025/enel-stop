mod configuration;

use config::{Config, FileFormat};
use log::{debug, info};
use rss::{Category, Channel, Item};
use std::{thread, time::Duration};

use crate::configuration::ServiceConfiguration;

pub fn start_service(file_path: &str) {
    let config = Config::builder()
        .add_source(config::File::new(file_path, FileFormat::Toml))
        .add_source(config::Environment::default())
        .build()
        .unwrap();

    debug!("---- Environment variables ----");
    for v in std::env::vars() {
        debug!("{} = {}", v.0, v.1)
    }

    let config_result = ServiceConfiguration::new(&config);

    match config_result {
        Err(err) => {
            panic!("There was an error when loading the configuration: {}", err)
        }
        Ok(config) => {
            start_loop(&config);
        }
    }
}

fn start_loop(config: &ServiceConfiguration) {
    let filtering_categs = convert_config_categs(&config.categories);
    info!("Filtering for categs: {:?}", filtering_categs);

    loop {
        let all_incidents = retrieve_incidents(&config.url);
        let filtered_incidents = filter_incidents(&all_incidents, &filtering_categs);
        if filtered_incidents.len() > 1 {
            debug!("Found: {}", all_incidents.len());
            send_sms(&all_incidents);
        }

        thread::sleep(Duration::from_millis(config.refresh_ms));
    }
}

fn retrieve_incidents(url: &str) -> Vec<Item> {
    let content = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let channel = Channel::read_from(&content[..]).unwrap();
    debug!("Scheduled downtime locations: {}", channel.items.len());

    channel.items
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

fn send_sms(locations_counter: &Vec<Item>) {
    for x in locations_counter {
        info!("Location: {}", x.title.as_ref().unwrap());
        // println!("Item: {}", x);
    }
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
