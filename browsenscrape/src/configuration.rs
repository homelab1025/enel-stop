use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use config::{Config, FileFormat};
use log::{debug, error};

const CONFIG_URL: &str = "service.url";
const CONFIG_FILTER_CATEGORIES: &str = "filter.categories";
const CONFIG_REDIS_SERVER: &str = "service.redis_server";

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceConfiguration {
    pub url: String,
    pub categories: Vec<String>,
    pub redis_server: Option<String>,
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
            redis_server: config.get_string(CONFIG_REDIS_SERVER).ok(),
        };

        Ok(service_configuration)
    }
}

impl Display for ServiceConfiguration {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "\nurl: {}\ncategories: {:?}", self.url, self.categories)
    }
}

pub fn get_configuration(config_cli_arg: &str) -> Result<ServiceConfiguration, &'static str> {
    let file_exists = std::path::Path::new(config_cli_arg).exists();

    if !file_exists {
        return Result::Err("Configuration file does not exist!");
    }

    let raw_config = Config::builder()
        .add_source(config::File::new(config_cli_arg, FileFormat::Toml))
        .add_source(config::Environment::default().separator("__"))
        .build()
        .map_err(|_err| "Could not parse configuration file.");

    debug!("---- Environment variables ----");
    for env_var in std::env::vars() {
        debug!("{} = {}", env_var.0, env_var.1)
    }

    raw_config.and_then(|c| {
        ServiceConfiguration::new(&c).map_err(|e| {
            // TODO: how to handle these errors?
            error!("Configuration init error: {}", e);
            "Could not build service configuration struct."
        })
    })
}

#[cfg(test)]
mod configuration_tests {
    use config::Config;

    use crate::configuration::CONFIG_REDIS_SERVER;

    use super::{ServiceConfiguration, CONFIG_FILTER_CATEGORIES, CONFIG_URL};

    #[test]
    fn config_loads_correctly() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "http://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REDIS_SERVER, "redis"))
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample).unwrap();

        let expected_config = ServiceConfiguration {
            url: "http://google.com".to_string(),
            categories: vec!["first".to_string(), "second".to_string()],
            redis_server: Some("redis".to_string()),
        };

        assert_eq!(service_config, expected_config);
    }

    #[test]
    fn config_fails_refresh_negative() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "http://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REDIS_SERVER, "redis"))
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
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample);

        assert!(service_config.is_err())
    }
}
