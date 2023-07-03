use std::io::Read;
use log::{debug, error, info};
use reqwest::blocking::Client;
use rss::{Category, Channel, Item};

pub fn parse_rss(url: &str, filter_categs: &Vec<Category>, rss_client: &Client) -> Vec<Item> {
    info!("Filtering for categs: {:?}", filter_categs);

    let channel_resp = rss_client.get(url).send();

    match channel_resp {
        Ok(mut resp) => {
            let mut buffer = Vec::new();
            let read_result = resp.read_to_end(&mut buffer);
            if read_result.is_err() {
                error!("There was an error reading the RSS into the buffer: {}", read_result.unwrap_err());
                return vec![];
            }

            let channel = match Channel::read_from(&buffer[..]) {
                Ok(channel) => channel,
                Err(err) => {
                    error!("There was an error parsing the RSS: {}", err);
                    return vec![];
                }
            };

            let filtered_items = filter_incidents(&channel.items, filter_categs);

            return filtered_items;

        }
        Err(err) => {
            error!("There was an error making the request for the RSS: {}", err);

            return vec![];
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

#[cfg(test)]
mod rss_reader_tests {
    use rss::{Category, ItemBuilder};
    use crate::rss_reader::filter_incidents;

    #[test]
    fn filter_incidents_correctly() {
        const FILTER_CATEG_1: &str = "one";
        const FILTER_CATEG_2: &str = "two";

        let filtering_categs = [FILTER_CATEG_1.to_string(), FILTER_CATEG_2.to_string()]
                .map(|x| Category {
                    domain: None,
                    name: String::from(x),
                }).to_vec();

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