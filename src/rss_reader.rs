use log::{error, info};
use reqwest::blocking::Client;
use rss::{Category, Channel, Item};
use std::io::Read;

pub fn parse_rss(url: &str, filter_categs: &Vec<String>) -> Vec<Item> {
    info!("Filtering for categs: {:?}", filter_categs);

    let client = Client::builder().cookie_store(true).build();

    match client {
        Ok(rss_client) => {
            let channel_resp = rss_client.get(url).send();

            match channel_resp {
                Ok(mut resp) => {
                    let mut buffer = Vec::new();
                    let read_result = resp.read_to_end(&mut buffer);
                    if read_result.is_err() {
                        error!(
                            "There was an error reading the RSS into the buffer: {}",
                            read_result.unwrap_err()
                        );
                        return vec![];
                    }

                    let channel = match Channel::read_from(&buffer[..]) {
                        Ok(channel) => channel,
                        Err(err) => {
                            error!("There was an error parsing the RSS: {}", err);
                            return vec![];
                        }
                    };

                    filter_incidents(&channel.items, &convert_config_categs(filter_categs))
                }
                Err(err) => {
                    if err.is_builder() {
                        panic!("The request can not be built: {}", err);
                    }

                    error!("There was an error making the request for the RSS: {}", err);
                    vec![]
                }
            }
        }
        Err(err) => {
            panic!("Can not instantiate RSS client: {}", err)
        }
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
mod rss_reader_tests {
    use crate::rss_reader::{convert_config_categs, filter_incidents};
    use rss::{Category, ItemBuilder};

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

        let filtering_categs = [FILTER_CATEG_1.to_string(), FILTER_CATEG_2.to_string()]
            .map(|x| Category {
                domain: None,
                name: x,
            })
            .to_vec();

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
