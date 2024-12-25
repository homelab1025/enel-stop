use core::panic;
use std::{env, error::Error};

use common::Record;
use config::{Config, FileFormat};
use configuration::ServiceConfiguration;
use headless_chrome::{browser, Browser};
use log::{debug, info, warn, LevelFilter, Record};
use simple_logger::SimpleLogger;

mod configuration;
mod rss_reader;

fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let cli_arg = env::args().nth(1);
    let config = cli_arg
        .map(|file_path| get_configuration(&file_path))
        .unwrap()
        .unwrap();

    const URL: &str = "https://www.reteleelectrice.ro/intreruperi/programate/";
    const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36";
    const XPATH: &str = "//*[@id='page-wrap']/div/div/div/div/a";

    let browser_result = Browser::default();
    let browser_tab = browser_result.and_then(|b| b.new_tab());

    match browser_tab {
        Ok(tab) => {
            tab.set_user_agent(USER_AGENT, None, None);
            tab.navigate_to(URL);

            tab.wait_for_xpath(XPATH)
                .map_err(|_| "Could not find xpath.")
                .and_then(|el| get_rss_href(el))
                .inspect(|link| {
                    tab.navigate_to(link)
                        .map_err(|_e| "Could not navigate to the RSS link");
                });

            let content = tab
                .get_content()
                .map_err(|_| "Could not get content")
                .map(|c| extract_items(c, Vec::new()));
        }
        Err(_e) => panic!("Could not open browser tab."),
    }

    // match browser_result {
    //     Ok(browser) => {
    // let browser_tab = browser.new_tab();
    // if browser_tab.is_err() {
    //     panic!("Can not open browser tab.")
    // }
    //
    // let browser_tab = browser_tab.unwrap();
    // let set_user_agent_result = browser_tab.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36", None,None);
    // if set_user_agent_result.is_err() {
    //     panic!(
    //         "Could not set the user agent: {}",
    //         set_user_agent_result.err().unwrap()
    //     )
    // }
    //
    // let navigation_result = browser_tab.navigate_to(URL);
    // if navigation_result.is_err() {
    //     panic!("Could not navigate to {}", URL);
    // }
    //
    // let element = browser_tab.wait_for_xpath("//*[@id='page-wrap']/div/div/div/div/a");
    // match element {
    //     Ok(extracted_element) => {
    //         let rss_href = get_rss_href(extracted_element);
    //         match rss_href {
    //             Ok(value) => {
    //                 info!("Found href: {}", value);
    //                 let navigation_result = browser_tab.navigate_to(value.as_ref());
    //
    //                 match navigation_result {
    //                     Ok(_result) => {
    //                         let items = extract_items(
    //                             browser_tab.get_content().unwrap(),
    //                             Vec::new(),
    //                         );
    //                     }
    //                     Err(error) => {
    //                         panic!("Can not get the RSS content: {}", error)
    //                     }
    //                 }
    //             }
    //             Err(error) => {
    //                 panic!("Could not get the link to the RSS file: {}", error);
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         panic!("Could not get the RSS link element: {}", err)
    //     }
    // }
    //     }
    //     Err(err) => {
    //         panic!("Can not instantiate browser: {}", err);
    //     }
    // }
}

fn get_configuration(config_cli_arg: &str) -> Result<ServiceConfiguration, &'static str> {
    let file_exists = std::path::Path::new(config_cli_arg).exists();

    if !file_exists {
        return Result::Err("Configuration file does not exist!");
    }

    let raw_config = Config::builder()
        .add_source(config::File::new(&config_cli_arg, FileFormat::Toml))
        .add_source(config::Environment::default().separator("__"))
        .build()
        .map_err(|_err| "Could not parse configuration file.");

    debug!("---- Environment variables ----");
    for env_var in std::env::vars() {
        debug!("{} = {}", env_var.0, env_var.1)
    }

    let config = raw_config.and_then(|c| {
        ServiceConfiguration::new(&c).map_err(|_e| "Could not build service configuration struct.")
    });
    return config;
}

fn extract_items(rss_content: String, filter_categs: &Vec<String>) -> Vec<Record> {
    let all_items = rss_reader::parse_rss(&rss_content, &vec!["Ilfov".to_string()]);
    vec![]
}

fn get_rss_href(extracted_element: headless_chrome::Element<'_>) -> Result<String, &'static str> {
    match extracted_element.get_attribute_value("href") {
        Ok(Some(href)) => Ok(href),
        Ok(None) => Err("No href was set."),
        Err(_error) => Err("could not extract the element attribute href"),
    }
}
