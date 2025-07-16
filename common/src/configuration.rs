use config::{Config, ConfigError, FileFormat};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

const CONFIG_URL: &str = "service.url";
const CONFIG_FILTER_CATEGORIES: &str = "filter.categories";
const CONFIG_REDIS_SERVER: &str = "service.redis_server";
const CONFIG_PUSHGATEWAY_SERVER: &str = "service.pushgateway_server";
const CONFIG_HTTP_PORT: &str = "service.http_port";
const CONFIG_CORS_PERMISSIVE: &str = "service.cors_permissive";
const CONFIG_LOG_LEVEL: &str = "service.log_level";
const CONFIG_DB_HOST: &str = "service.db_host";
const CONFIG_DB_PORT: &str = "service.db_port";
const CONFIG_DB_NAME: &str = "service.db_name";
const CONFIG_DB_USERNAME: &str = "service.db_username";
const CONFIG_DB_PASSWORD: &str = "service.db_password";

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ServiceConfiguration {
    pub url: String,
    pub categories: Vec<String>,
    pub redis_server: Option<String>,
    pub pushgateway_server: Option<String>,
    pub http_port: u32,
    pub cors_permissive: bool,
    pub log_level: String,
    pub db_host: Option<String>,
    pub db_port: Option<u32>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_name: Option<String>,
}

pub struct ServiceConfigurationBuilder {
    url: Option<String>,
    categories: Vec<String>,
    redis_server: Option<String>,
    pushgateway_server: Option<String>,
    http_port: u32,
    cors_permissive: bool,
    log_level: String,
    db_host: Option<String>,
    db_port: Option<u32>,
    db_user: Option<String>,
    db_password: Option<String>,
    db_name: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ConfigurationError {
    message: String,
}
impl ConfigurationError {
    fn from_str(message: &str) -> ConfigurationError {
        ConfigurationError {
            message: message.to_string(),
        }
    }

    fn from_string(message: String) -> ConfigurationError {
        ConfigurationError {
            message: message.to_string(),
        }
    }
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ConfigurationError {}

impl From<ConfigError> for ConfigurationError {
    fn from(err: ConfigError) -> Self {
        ConfigurationError::from_str(&err.to_string())
    }
}

impl Default for ServiceConfigurationBuilder {
    fn default() -> Self {
        ServiceConfigurationBuilder {
            url: None, // Mandatory, so starts as None
            categories: Vec::new(),
            redis_server: None,
            pushgateway_server: None,
            http_port: 8080,               // Default value
            cors_permissive: false,        // Default value
            log_level: "info".to_string(), // Default value
            db_host: None,
            db_port: None,
            db_user: None,
            db_password: None,
            db_name: None,
        }
    }
}

impl ServiceConfigurationBuilder {
    /// Sets the URL for the service configuration. This field is mandatory.
    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    /// Sets the categories for the service configuration.
    pub fn categories(&mut self, categories: Vec<String>) -> &mut Self {
        self.categories = categories;
        self
    }

    /// Adds a single category to the service configuration.
    pub fn add_category(&mut self, category: String) -> &mut Self {
        self.categories.push(category);
        self
    }

    /// Sets the Redis server address.
    pub fn redis_server(&mut self, redis_server: String) -> &mut Self {
        self.redis_server = Some(redis_server);
        self
    }

    /// Sets the Pushgateway server address.
    pub fn pushgateway_server(&mut self, pushgateway_server: String) -> &mut Self {
        self.pushgateway_server = Some(pushgateway_server);
        self
    }

    /// Sets the HTTP port.
    pub fn http_port(&mut self, http_port: u32) -> &mut Self {
        self.http_port = http_port;
        self
    }

    /// Sets whether CORS is permissive.
    pub fn cors_permissive(&mut self, cors_permissive: bool) -> &mut Self {
        self.cors_permissive = cors_permissive;
        self
    }

    /// Sets the log level.
    pub fn log_level(&mut self, log_level: String) -> &mut Self {
        self.log_level = log_level;
        self
    }

    /// Sets the database host.
    pub fn db_host(&mut self, db_host: String) -> &mut Self {
        self.db_host = Some(db_host);
        self
    }

    /// Sets the database port.
    pub fn db_port(&mut self, db_port: u32) -> &mut Self {
        self.db_port = Some(db_port);
        self
    }

    /// Sets the database user.
    pub fn db_user(&mut self, db_user: String) -> &mut Self {
        self.db_user = Some(db_user);
        self
    }

    /// Sets the database password.
    pub fn db_password(&mut self, db_password: String) -> &mut Self {
        self.db_password = Some(db_password);
        self
    }

    /// Sets the database name.
    pub fn db_name(&mut self, db_name: String) -> &mut Self {
        self.db_name = Some(db_name);
        self
    }

    /// Builds the `ServiceConfiguration` instance.
    /// Returns an `Err` if the mandatory `url` field has not been set.
    pub fn build(self) -> Result<ServiceConfiguration, ConfigurationError> {
        let url = self
            .url
            .ok_or(ConfigurationError::from_str("URL is mandatory and must be set."))?;

        Ok(ServiceConfiguration {
            url,
            categories: self.categories,
            redis_server: self.redis_server,
            pushgateway_server: self.pushgateway_server,
            http_port: self.http_port,
            cors_permissive: self.cors_permissive,
            log_level: self.log_level,
            db_host: self.db_host,
            db_port: self.db_port,
            db_user: self.db_user,
            db_password: self.db_password,
            db_name: self.db_name,
        })
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

pub fn get_configuration(config_cli_arg: &str) -> Result<ServiceConfiguration, ConfigurationError> {
    let file_exists = std::path::Path::new(config_cli_arg).exists();

    if !file_exists {
        return Err(ConfigurationError::from_str("Configuration file does not exist"));
    }

    let raw_config = Config::builder()
        .add_source(config::File::new(config_cli_arg, FileFormat::Toml))
        .add_source(
            config::Environment::default()
                .prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .map_err(|x| ConfigurationError::from_string(x.to_string()))?;

    convert_configuration(&raw_config)
}

fn convert_configuration(raw_config: &Config) -> Result<ServiceConfiguration, ConfigurationError> {
    let categories = raw_config
        .get_array(CONFIG_FILTER_CATEGORIES)?
        .into_iter()
        .map(|x| x.into_string().unwrap())
        .collect();

    let mut config_builder = ServiceConfigurationBuilder::default();

    config_builder
        .url(raw_config.get_string(CONFIG_URL)?)
        .log_level(raw_config.get_string(CONFIG_LOG_LEVEL)?)
        .redis_server(raw_config.get_string(CONFIG_REDIS_SERVER)?)
        .cors_permissive(raw_config.get_bool(CONFIG_CORS_PERMISSIVE)?)
        .http_port(raw_config.get::<u32>(CONFIG_HTTP_PORT)?)
        .categories(categories)
        .pushgateway_server(raw_config.get_string(CONFIG_PUSHGATEWAY_SERVER)?);

    let _ = raw_config.get_string(CONFIG_DB_HOST).inspect(|value| {
        config_builder.db_host(value.clone());
    });
    let _ = raw_config.get_string(CONFIG_DB_NAME).inspect(|value| {
        config_builder.db_name(value.clone());
    });
    let _ = raw_config.get::<u32>(CONFIG_DB_PORT).inspect(|value| {
        config_builder.db_port(*value);
    });
    let _ = raw_config.get_string(CONFIG_DB_USERNAME).inspect(|value| {
        config_builder.db_user(value.clone());
    });
    let _ = raw_config.get_string(CONFIG_DB_PASSWORD).inspect(|value| {
        config_builder.db_password(value.clone());
    });

    config_builder.build()
}

#[cfg(test)]
mod configuration_tests {
    use config::Config;

    use crate::configuration::{
        convert_configuration, ConfigurationError, ServiceConfigurationBuilder, CONFIG_CORS_PERMISSIVE, CONFIG_HTTP_PORT,
        CONFIG_LOG_LEVEL, CONFIG_PUSHGATEWAY_SERVER, CONFIG_REDIS_SERVER,
    };

    use super::{ServiceConfiguration, CONFIG_FILTER_CATEGORIES, CONFIG_URL};
    #[test]
    fn test_service_configuration_builder_minimal() {
        let mut builder = ServiceConfigurationBuilder::default();
        builder.url("http://localhost:8000".to_string());

        let config = builder.build().unwrap();

        assert_eq!(config.url, "http://localhost:8000");
        assert_eq!(config.categories.len(), 0);
        assert_eq!(config.redis_server, None);
        assert_eq!(config.http_port, 8080);
        assert_eq!(config.cors_permissive, false);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_service_configuration_builder_full() {
        let mut builder = ServiceConfigurationBuilder::default();
        builder
            .url("http://api.example.com".to_string())
            .categories(vec!["web".to_string(), "api".to_string()])
            .redis_server("redis://127.0.0.1:6379".to_string())
            .pushgateway_server("http://prometheus:9091".to_string())
            .http_port(9000)
            .cors_permissive(true)
            .log_level("debug".to_string())
            .db_host("db.example.com".to_string())
            .db_port(5432)
            .db_user("admin".to_string())
            .db_password("secret".to_string())
            .db_name("myapp_db".to_string());
        let config = builder.build().unwrap();

        assert_eq!(config.url, "http://api.example.com");
        assert_eq!(config.categories, vec!["web", "api"]);
        assert_eq!(config.redis_server, Some("redis://127.0.0.1:6379".to_string()));
        assert_eq!(config.pushgateway_server, Some("http://prometheus:9091".to_string()));
        assert_eq!(config.http_port, 9000);
        assert_eq!(config.cors_permissive, true);
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.db_host, Some("db.example.com".to_string()));
        assert_eq!(config.db_port, Some(5432));
        assert_eq!(config.db_user, Some("admin".to_string()));
        assert_eq!(config.db_password, Some("secret".to_string()));
        assert_eq!(config.db_name, Some("myapp_db".to_string()));
    }

    #[test]
    fn test_service_configuration_builder_add_category() {
        let mut builder = ServiceConfigurationBuilder::default();
        builder
            .url("http://test.com".to_string())
            .add_category("test1".to_string())
            .add_category("test2".to_string());
        let config = builder.build().unwrap();

        assert_eq!(config.categories, vec!["test1", "test2"]);
    }

    #[test]
    fn test_service_configuration_builder_missing_url() {
        let config_result = ServiceConfigurationBuilder::default().build();
        assert!(config_result.is_err());
        assert_eq!(
            config_result.unwrap_err(),
            ConfigurationError::from_str("URL is mandatory and must be set.")
        );
    }
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

        let service_config = convert_configuration(&config_sample).unwrap();

        let expected_config = ServiceConfiguration {
            url: "https://google.com".to_string(),
            categories: vec!["first".to_string(), "second".to_string()],
            redis_server: Some("redis".to_string()),
            pushgateway_server: Some("pushgateway".to_string()),
            http_port: 8090,
            cors_permissive: true,
            log_level: "debug".to_string(),
            db_host: None,
            db_password: None,
            db_user: None,
            db_port: None,
            db_name: None,
        };

        assert_eq!(service_config, expected_config);
    }
}
