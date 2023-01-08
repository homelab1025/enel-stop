use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use config::Config;
use url::Url;

const CONFIG_URL: &str = "service.url";
const CONFIG_REFRESH_MS: &str = "service.refresh_ms";
const CONFIG_FILTER_CATEGORIES: &str = "filter.categories";
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
    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let service_configuration = Self {
            url: config.get_string(CONFIG_URL)?,
            categories: config.get_array(CONFIG_FILTER_CATEGORIES).unwrap().into_iter().map(|x| { x.into_string().unwrap() }).collect(),
            refresh_ms: config.get_string(CONFIG_REFRESH_MS)?.parse::<u64>()?,
            auth_token: String::from("no token"),
            phone_numer: String::from("no phone"),
        };

        Url::parse(&service_configuration.url)?;

        Ok(service_configuration)
    }
}

impl Display for ServiceConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nurl: {}\ncategories: {:?}\nrefresh_ms: {}", self.url, self.categories, self.refresh_ms)
    }
}
