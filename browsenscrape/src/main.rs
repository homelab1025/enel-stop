use core::panic;
use std::{
    env,
    path::{Path, PathBuf},
};

use headless_chrome::{Browser, LaunchOptionsBuilder};
use log::{debug, error, info, LevelFilter};
use redis::{Commands, RedisError};
use rss_reader::parse_rss;
use simple_logger::SimpleLogger;

mod configuration;
mod rss_reader;

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36";
const XPATH: &str = "//*[@id='page-wrap']/div/div/div/div/a";

const CHROMIUM_DRIVER_PATH: &str = "/usr/bin/chromium";

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

    let cli_arg = env::args().nth(1);
    let config = cli_arg.map(|file_path| configuration::get_configuration(&file_path));

    if config.is_none() {
        panic!("No configuration has been provided.");
    }

    let config = config.unwrap();
    if config.is_err() {
        panic!("some other config issue: {}", config.unwrap_err());
    }
    let config = config.unwrap();

    info!("Configuration: {}", config);

    let mut redis_connection = match config.redis_server {
        Some(conn_string) => {
            let client = redis::Client::open(conn_string)
                .expect("Redis client could not be created. Check connection string or remove it if you don't want to store results.");

            Some(
                client
                    .get_connection()
                    .expect("Could not create connection, even if redis client was created."),
            )
        }
        None => None,
    };

    let chromium_path = match Path::new(CHROMIUM_DRIVER_PATH).exists() {
        true => Some(PathBuf::from(CHROMIUM_DRIVER_PATH)),
        false => None,
    };

    let browser_result = Browser::new(
        LaunchOptionsBuilder::default()
            .enable_logging(true)
            .sandbox(false)
            .path(chromium_path)
            .build()
            .unwrap(),
    )
    .unwrap();

    let browser_tab = browser_result.new_tab();

    match browser_tab {
        Ok(tab) => {
            let rss_content = get_rss_content(&config.url, tab);

            let incidents = parse_rss(&rss_content, &config.categories);

            debug!("Incidents: {:?}", incidents);

            if let Some(conn) = redis_connection.as_mut() {
                let stored_incidents = incidents
                    .iter()
                    .map(|incident| {
                        let ser_inc = match serde_json::to_string(&incident) {
                            Err(e) => return Err(e.to_string()),
                            Ok(ser_res) => ser_res,
                        };

                        let redis_result: Result<String, RedisError> = conn.set(&incident.id, ser_inc);

                        match redis_result {
                            Err(e) => {
                                error!("Could not store incident: {}", e);
                                Err(e.to_string())
                            }
                            Ok(rr) => Ok(rr),
                        }
                    })
                    .filter(|incident_result| incident_result.is_ok())
                    .count();

                info!(
                    "Stored {} incidents out of {} received.",
                    stored_incidents,
                    incidents.len()
                );
            }
        }
        Err(_e) => panic!("Could not open browser tab."),
    }
}

fn get_rss_content(starting_url: &str, tab: std::sync::Arc<headless_chrome::Tab>) -> String {
    let navigation = navigate_to_rss(starting_url, &tab);
    if navigation.is_err() {
        let err_msg = navigation.unwrap_err();
        panic!("Failed navigation: {err_msg}")
    }

    let _res = tab.wait_until_navigated();

    let rss_content = match tab.find_element("#webkit-xml-viewer-source-xml") {
        Ok(real_rss_content) => real_rss_content
            .call_js_fn(
                r#"
                            function getInnerHtml() {
                                return this.innerHTML
                            }
                            "#,
                vec![],
                true,
            )
            .unwrap()
            .value
            .unwrap()
            .to_string(),
        Err(_) => tab.get_content().unwrap(),
    };
    rss_content
}

// TODO: get rid of the Arc parameter? Should I though?
fn navigate_to_rss(url: &str, tab: &std::sync::Arc<headless_chrome::Tab>) -> Result<(), String> {
    tab.set_user_agent(USER_AGENT, None, None)
        .map_err(|e| format!("Could not set user agent due to: {}", e))?;

    tab.navigate_to(url).map_err(|_e| "Could not navigate to homepage.")?;

    let rss_href = tab
        .wait_for_xpath(XPATH)
        .map_err(|_| "Could not find xpath.")
        .and_then(|el| get_rss_href(el));

    if rss_href.is_ok() {
        tab.navigate_to(rss_href?.as_str())
            .map_err(|_e| "Could not navigate to the RSS link")?;
    } else {
        return Err("Could not extrat the rss href".to_string());
    }
    Ok(())
}

fn get_rss_href(extracted_element: headless_chrome::Element<'_>) -> Result<String, &'static str> {
    match extracted_element.get_attribute_value("href") {
        Ok(Some(href)) => Ok(href),
        Ok(None) => Err("No href was set."),
        Err(_error) => Err("could not extract the element attribute href"),
    }
}
