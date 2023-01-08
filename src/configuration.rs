use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use config::{Config, FileFormat};
use url::Url;

const CONFIG_URL: &str = "service.url";
const CONFIG_REFRESH_SEC: &str = "service.refresh_ms";
#[allow(unused)]
const CONFIG_TWILIO_TOKEN: &str = "";
#[allow(unused)]
const CONFIG_TWILIO_PHONE: &str = "refresh_ms";

#[derive(Debug, Clone)]
pub struct ServiceConfiguration {
    pub url: String,
    pub categories: Vec<String>,
    pub refresh_ms: u64,
    pub auth_token: String,
    pub phone_numer: String,
}

impl ServiceConfiguration {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let config = Config::builder()
            .add_source(config::File::new(file_path, FileFormat::Toml))
            .build()?;

        let service_configuration = Self {
            url: config.get_string(CONFIG_URL)?,
            categories: config.get_array("filter.categories").unwrap().into_iter().map(|x| { x.into_string().unwrap() }).collect(),
            refresh_ms: config.get_string(CONFIG_REFRESH_SEC)?.parse::<u64>()?,
            auth_token: String::from("no token"),
            phone_numer: String::from("no phone"),
        };

        Url::parse(&service_configuration.url)?;

        Ok(service_configuration)
    }
}

impl Display for ServiceConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "url: {}", self.url);
        writeln!(f, "categories: {:?}", self.categories)
    }
}
