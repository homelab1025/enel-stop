use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use config::Config;

const CONFIG_URL: &str = "service.url";
const CONFIG_REFRESH_MS: &str = "service.refresh_ms";
const CONFIG_FILTER_CATEGORIES: &str = "filter.categories";
const CONFIG_REDIS_SERVER: &str = "service.redis_server";

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceConfiguration {
    pub url: String,
    pub categories: Vec<String>,
    pub refresh_ms: u64,
    pub redis_server: String,
}

impl ServiceConfiguration {
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let service_configuration = Self {
            url: config.get_string(CONFIG_URL)?,
            categories: config
                .get_array(CONFIG_FILTER_CATEGORIES)
                .unwrap()
                .into_iter()
                .map(|x| x.into_string().unwrap())
                .collect(),
            refresh_ms: config.get_string(CONFIG_REFRESH_MS)?.parse::<u64>()?,
            redis_server: config.get_string(CONFIG_REDIS_SERVER)?,
        };

        Ok(service_configuration)
    }
}

impl Display for ServiceConfiguration {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            formatter,
            "\nurl: {}\ncategories: {:?}\nrefresh_ms: {}",
            self.url, self.categories, self.refresh_ms
        )
    }
}

#[cfg(test)]
mod configuration_tests {
    use config::Config;

    use crate::configuration::CONFIG_REFRESH_MS;

    use super::{ServiceConfiguration, CONFIG_FILTER_CATEGORIES, CONFIG_URL};

    #[test]
    fn config_loads_correctly() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "http://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REFRESH_MS, 30))
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample).unwrap();

        let expected_config = ServiceConfiguration {
            url: "http://google.com".to_string(),
            refresh_ms: 30,
            categories: vec!["first".to_string(), "second".to_string()],
            redis_server: "localhost-redis".to_string(),
        };

        assert_eq!(service_config, expected_config);
    }

    #[test]
    fn config_fails_refresh_negative() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "http://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REFRESH_MS, -30))
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample);

        assert!(service_config.is_err())
    }

    #[test]
    fn config_fails_refresh_notint() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "http://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REFRESH_MS, 30.666))
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample);

        assert!(service_config.is_err())
    }
}
