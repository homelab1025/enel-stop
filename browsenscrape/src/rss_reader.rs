use common::Record;
use log::{debug, error, info};
use regex::Regex;
use rss::{Category, Channel};

pub fn parse_rss(rss_content: &str, filter_categs: &Vec<String>) -> Vec<Record> {
    info!("Filtering for categs: {:?}", filter_categs);

    debug!("Content: {}", rss_content);

    let channel = match Channel::read_from(rss_content.as_bytes()) {
        Ok(channel) => channel,
        Err(err) => {
            error!("There was an error parsing the RSS: {}", err);
            return vec![];
        }
    };

    let location_extract_pattern = r"(.*?) Judet: (\w+)\s+Localitate: (.+)";
    let location_extractor = Regex::new(location_extract_pattern).unwrap();
    let converted_filters = convert_config_categs(filter_categs);

    channel
        .items()
        .iter()
        .filter(|item| check_categories(item, &converted_filters))
        .filter_map(|item| convert_item(item, &location_extractor))
        .collect()
}

fn check_categories(item: &rss::Item, converted_filters: &[Category]) -> bool {
    converted_filters
        .iter()
        .all(|needle| item.categories.contains(needle))
}

fn convert_item(rss_item: &rss::Item, location_extractor: &Regex) -> Option<Record> {
    let title = rss_item.title.as_ref()?;

    location_extractor.captures(title).and_then(|capture| {
        let judet = capture.get(2).unwrap().as_str();
        let localitate = capture.get(3)?.as_str();
        let id = rss_item.guid.as_ref()?;

        Option::Some(Record {
            id: id.value.to_string(),
            judet: judet.to_string(),
            localitate: localitate.to_string(),
            title: rss_item.title.as_ref()?.to_string(),
            description: rss_item.description.as_ref()?.to_string(),
        })
    })
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
    use crate::rss_reader::{check_categories, convert_config_categs};
    use rss::{Category, ItemBuilder};

    const FILTER_CATEG_1: &str = "one";
    const FILTER_CATEG_2: &str = "two";

    fn generate_categories() -> Vec<Category> {
        [FILTER_CATEG_1.to_string(), FILTER_CATEG_2.to_string()]
            .iter()
            .clone()
            .map(|f| Category {
                domain: None,
                name: f.to_string(),
            })
            .collect()
    }

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
    fn filter_incidents_no_correct_categ() {
        let filtering_categs = generate_categories();

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

        let result = check_categories(
            &ItemBuilder::default().categories(incorrect_cats).build(),
            &filtering_categs,
        );
        assert!(!result);
    }

    #[test]
    fn filter_incidents_partial() {
        let filtering_categs = generate_categories();

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

        let result = check_categories(
            &ItemBuilder::default()
                .categories(partial_correct_cats)
                .build(),
            &filtering_categs,
        );
        assert!(!result);
    }

    #[test]
    fn filter_incidents_single_cat() {
        let filtering_categs = generate_categories();

        let single_cats = vec![Category {
            domain: None,
            name: FILTER_CATEG_1.to_string(),
        }];

        let result = check_categories(
            &ItemBuilder::default().categories(single_cats).build(),
            &filtering_categs,
        );
        assert!(!result);
    }

    #[test]
    fn filter_incidents_correct() {
        let filtering_categs = generate_categories();

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

        let result = check_categories(
            &ItemBuilder::default().categories(correct_cats).build(),
            &filtering_categs,
        );
        assert!(result);
    }
}
