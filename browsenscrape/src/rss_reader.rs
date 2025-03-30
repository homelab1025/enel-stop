use chrono::NaiveDate;
use common::Record;
use log::{debug, error, info};
use regex::Regex;
use rss::{Category, Channel};

const LOCATION_PATTERN: &str = r"(.*?) Judet: (\w+)\s+Localitate: (.+)";

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

    let converted_filters = convert_config_categs(filter_categs);

    filter_items(channel.items(), converted_filters)
}

fn filter_items(items: &[rss::Item], converted_filters: Vec<Category>) -> Vec<Record> {
    let location_extractor = Regex::new(LOCATION_PATTERN).unwrap();

    items
        .iter()
        .filter(|item| check_categories(item, &converted_filters))
        .filter_map(|item| convert_item(item, &location_extractor))
        .collect()
}

fn check_categories(item: &rss::Item, converted_filters: &[Category]) -> bool {
    converted_filters.iter().all(|needle| item.categories.contains(needle))
}

fn convert_item(rss_item: &rss::Item, location_extractor: &Regex) -> Option<Record> {
    let title = rss_item.title.as_ref()?;

    location_extractor.captures(title).and_then(|capture| {
        let judet = capture.get(2).unwrap().as_str();
        let localitate = capture.get(3)?.as_str();
        let id = rss_item.guid.as_ref()?;

        let title_parsing_result = NaiveDate::parse_and_remainder(title, "%d.%m.%Y");

        let incident_datetime = match title_parsing_result {
            Ok((incident_datetime, _remaining)) => Some(incident_datetime),
            Err(e) => {
                error!("Error when parsing the date from the title({}): {}", title, e);
                None
            }
        }?;

        Option::Some(Record {
            id: id.value.to_string(),
            date: incident_datetime,
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
    use chrono::NaiveDate;
    use common::Record;
    use regex::Regex;
    use rss::{Category, Guid, ItemBuilder};

    use super::{convert_item, filter_items, LOCATION_PATTERN};

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
    fn filter_success() {
        let items = [
            ItemBuilder::default()
                .title("WRONG title".to_string())
                .description("this should be skipped".to_string())
                .guid(Guid {
                    value: "123".to_string(),
                    permalink: false,
                })
                .categories(generate_categories())
                .build(),
            ItemBuilder::default()
                .title("02.03.2016 Judet: Bucuresti Localitate: Sector 5".to_string())
                .description("one of a kind".to_string())
                .guid(Guid {
                    permalink: false,
                    value: "62016".to_string(),
                })
                .categories(generate_categories())
                .build(),
        ];

        let result = filter_items(&items, generate_categories());
        assert_eq!(1, result.len());
    }

    #[test]
    fn convert_item_correct() {
        let description = "Something something good".to_string();
        let title = "21.02.1985 06:00 - 08:00 Judet: X Localitate: Y".to_string();
        println!("{}", title);
        let id = "123 - my id".to_string();
        let extractor = Regex::new(LOCATION_PATTERN).unwrap();

        let rss_item = ItemBuilder::default()
            .categories(vec![Category {
                domain: None,
                name: FILTER_CATEG_1.to_string(),
            }])
            .title(title.clone())
            .description(description.clone())
            .guid(Guid {
                permalink: false,
                value: id.clone(),
            })
            .build();

        let result = convert_item(&rss_item, &extractor);

        let expected_record = Record {
            id,
            // date: "1985-02-21".to_string(),
            date: NaiveDate::parse_from_str("1985-02-21", "%Y-%m-%d").unwrap(),
            judet: "X".to_string(),
            localitate: "Y".to_string(),
            description,
            title,
        };

        assert_eq!(expected_record, result.unwrap());
    }

    #[test]
    fn convert_item_fail_title_parse() {
        let description = "Something something good".to_string();
        let title = "21.02.1985 06:00 - 08:00 : X Localitate: Y".to_string();
        println!("{}", title);
        let id = "123 - my id".to_string();
        let extractor = Regex::new(LOCATION_PATTERN).unwrap();

        let rss_item = ItemBuilder::default()
            .categories(vec![Category {
                domain: None,
                name: FILTER_CATEG_1.to_string(),
            }])
            .title(title.clone())
            .description(description.clone())
            .guid(Guid {
                permalink: false,
                value: id.clone(),
            })
            .build();

        let result = convert_item(&rss_item, &extractor);

        assert_eq!(None, result);
    }

    #[test]
    fn convert_item_fail_date_parse() {
        let description = "Something something good".to_string();
        let title = "21198 Judet: X Localitate: Y".to_string();
        println!("{}", title);
        let id = "123 - my id".to_string();
        let extractor = Regex::new(LOCATION_PATTERN).unwrap();

        let rss_item = ItemBuilder::default()
            .categories(vec![Category {
                domain: None,
                name: FILTER_CATEG_1.to_string(),
            }])
            .title(title.clone())
            .description(description.clone())
            .guid(Guid {
                permalink: false,
                value: id.clone(),
            })
            .build();

        let result = convert_item(&rss_item, &extractor);

        assert_eq!(None, result);
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
            &ItemBuilder::default().categories(partial_correct_cats).build(),
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
