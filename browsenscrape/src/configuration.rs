use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use config::{Config, FileFormat};
use log::{debug, error, info};

const CONFIG_URL: &str = "service.url";
const CONFIG_REFRESH_MS: &str = "service.refresh_ms";
const CONFIG_FILTER_CATEGORIES: &str = "filter.categories";
const CONFIG_REDIS_SERVER: &str = "service.redis_server";
const CONFIG_STORE_ENABLED: &str = "service.store_enabled";

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceConfiguration {
    pub url: String,
    pub categories: Vec<String>,
    pub refresh_ms: u64,
    pub redis_server: String,
    pub store_enabled: bool,
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
            store_enabled: config.get_bool(CONFIG_STORE_ENABLED)?,
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

    use crate::configuration::{CONFIG_REDIS_SERVER, CONFIG_REFRESH_MS, CONFIG_STORE_ENABLED};

    use super::{ServiceConfiguration, CONFIG_FILTER_CATEGORIES, CONFIG_URL};

    #[test]
    fn config_loads_correctly() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "http://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REFRESH_MS, 30))
            .and_then(|x| x.set_default(CONFIG_REDIS_SERVER, "redis"))
            .and_then(|x| x.set_default(CONFIG_STORE_ENABLED, "true"))
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample).unwrap();

        let expected_config = ServiceConfiguration {
            url: "http://google.com".to_string(),
            refresh_ms: 30,
            categories: vec!["first".to_string(), "second".to_string()],
            redis_server: "redis".to_string(),
            store_enabled: true,
        };

        assert_eq!(service_config, expected_config);
    }

    #[test]
    fn config_fails_refresh_negative() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "http://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REFRESH_MS, -30))
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
            .and_then(|x| x.set_default(CONFIG_REFRESH_MS, 30.666))
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample);

        assert!(service_config.is_err())
    }
}
