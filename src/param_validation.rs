use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;

use config::{Config, ConfigError, FileFormat, FileSourceFile};
use url::{ParseError, Url};

use crate::param_validation::ArgumentsError::{ConfigFileError, InvalidUrl, MissingUrl};

pub enum ArgumentsError {
    MissingUrl,
    InvalidUrl(ParseError),
    ConfigFileError(ConfigError),
}

impl Debug for ArgumentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error(self, f)
    }
}

impl Display for ArgumentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_error(self, f)
    }
}

fn format_error(url_error: &ArgumentsError, f: &mut Formatter<'_>) -> std::fmt::Result {
    match url_error {
        MissingUrl => {
            write!(f, "The URL has no been specified.")
        }
        InvalidUrl(parse_error) => {
            write!(f, "The specified URL is invalid: {}", parse_error)
        }
        ConfigFileError(error) => {
            write!(f, "There was an error with the configuration file: {}", error)
        }
    }
}

impl Error for ArgumentsError {}

pub fn validate_params(cli_arg: Option<String>) -> Result<Url, ArgumentsError> {
    match cli_arg {
        Some(url_path) => {
            let rss_url = Url::parse(&url_path);

            match rss_url {
                Ok(url) => {
                    println!("Using URL: {}", url);
                    Ok(url)
                }
                Err(err) => Err(InvalidUrl(err))
            }
        }
        None => Err(MissingUrl)
    }
}


mod configuration {
    use config::{Config, ConfigError, FileFormat};

    const CONFIG_URL: &str = "url";
    const CONFIG_REFRESH_SEC: &str = "refresh_ms";
    const CONFIG_TWILIO_TOKEN: &str = "";
    const CONFIG_TWILIO_PHONE: &str = "refresh_ms";

    #[derive(Debug, Clone)]
    pub struct Service {
        pub url: String,
        pub refresh_ms: i64,
        pub auth_token: String,
        pub phone_numer: String,
    }

    impl Service {
        pub fn new(file_path: &str) -> Result<Self, ConfigError> {
            let config = Config::builder()
                .add_source(config::File::new(file_path, FileFormat::Toml))
                .build()?;

            Ok(Self {
                url: config.get_string(CONFIG_URL)?,
                refresh_ms: config.get_int(CONFIG_REFRESH_SEC)?,
                auth_token: String::from("no token"),
                phone_numer: String::from("no phone"),
            })
        }
    }
}
