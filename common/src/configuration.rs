use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use config::{Config, FileFormat};
use log::{debug, error};

const CONFIG_URL: &str = "service.url";
const CONFIG_FILTER_CATEGORIES: &str = "filter.categories";
const CONFIG_REDIS_SERVER: &str = "service.redis_server";
const CONFIG_PUSHGATEWAY_SERVER: &str = "service.pushgateway_server";
const CONFIG_HTTP_PORT: &str = "service.http_port";
const CONFIG_CORS_PERMISSIVE: &str = "service.cors_permissive";
const CONFIG_LOG_LEVEL: &str = "service.log_level";

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceConfiguration {
    pub url: String,
    pub categories: Vec<String>,
    pub redis_server: Option<String>,
    pub pushgateway_server: Option<String>,
    pub http_port: u32,
    pub cors_permissive: bool,
    pub log_level: String
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
            pushgateway_server: config.get_string(CONFIG_PUSHGATEWAY_SERVER).ok(),
            http_port: config.get::<u32>(CONFIG_HTTP_PORT)?,
            cors_permissive: config.get::<bool>(CONFIG_CORS_PERMISSIVE)?,
            log_level: config.get_string(CONFIG_LOG_LEVEL)?
        };

        Ok(service_configuration)
    }
}

impl Display for ServiceConfiguration {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            formatter,
            "\nurl: {}\ncategories: {:?}\nredis_server: {:?}\n pushgateway: {:?}\nhttp_port: {:?}\ncors_permissive: {:?}",
            self.url, self.categories, self.redis_server, self.pushgateway_server, self.http_port, self.cors_permissive
        )
    }
}

pub fn get_configuration(config_cli_arg: &str) -> Result<ServiceConfiguration, &'static str> {
    let file_exists = std::path::Path::new(config_cli_arg).exists();

    if !file_exists {
        return Err("Configuration file does not exist!");
    }

    let raw_config = Config::builder()
        .add_source(config::File::new(config_cli_arg, FileFormat::Toml))
        .add_source(
            config::Environment::default()
                .prefix("ENEL")
                .prefix_separator("_")
                .separator("__"),
        )
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

    use crate::configuration::{CONFIG_CORS_PERMISSIVE, CONFIG_HTTP_PORT, CONFIG_LOG_LEVEL, CONFIG_PUSHGATEWAY_SERVER, CONFIG_REDIS_SERVER};

    use super::{ServiceConfiguration, CONFIG_FILTER_CATEGORIES, CONFIG_URL};

    #[test]
    fn config_loads_correctly() {
        let config_sample = Config::builder()
            .set_default(CONFIG_URL, "https://google.com")
            .and_then(|x| x.set_default(CONFIG_FILTER_CATEGORIES, vec!["first", "second"]))
            .and_then(|x| x.set_default(CONFIG_REDIS_SERVER, "redis"))
            .and_then(|x| x.set_default(CONFIG_PUSHGATEWAY_SERVER, "pushgateway"))
            .and_then(|x| x.set_default(CONFIG_HTTP_PORT, 8090))
            .and_then(|x| x.set_default(CONFIG_CORS_PERMISSIVE, "true"))
            .and_then(|x| x.set_default(CONFIG_LOG_LEVEL, "debug"))
            .unwrap()
            .build()
            .unwrap();

        let service_config = ServiceConfiguration::new(&config_sample).unwrap();

        let expected_config = ServiceConfiguration {
            url: "https://google.com".to_string(),
            categories: vec!["first".to_string(), "second".to_string()],
            redis_server: Some("redis".to_string()),
            pushgateway_server: Some("pushgateway".to_string()),
            http_port: 8090,
            cors_permissive: true,
            log_level: "debug".to_string()
        };

        assert_eq!(service_config, expected_config);
    }
}
