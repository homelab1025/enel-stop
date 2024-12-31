use core::panic;
use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

use headless_chrome::{Browser, LaunchOptionsBuilder};
use log::{info, LevelFilter};
use rss_reader::parse_rss;
use simple_logger::SimpleLogger;

mod configuration;
mod rss_reader;

const URL: &str = "https://www.reteleelectrice.ro/intreruperi/programate/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36";
const XPATH: &str = "//*[@id='page-wrap']/div/div/div/div/a";

fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

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

    // const CHROMIUM_DRIVER_PATH: &str = "/usr/bin/chromedriver";
    // let chromium_path = match Path::new(CHROMIUM_DRIVER_PATH).exists() {
    //     true => Some(PathBuf::from(CHROMIUM_DRIVER_PATH)),
    //     false => None,
    // };

    let browser_result = Browser::new(
        LaunchOptionsBuilder::default()
            .enable_logging(true)
            .sandbox(false)
            // .path(chromium_path)
            .build()
            .unwrap(),
    )
    .unwrap();

    let browser_tab = browser_result.new_tab();

    match browser_tab {
        Ok(tab) => {
            let navigation = navigate_to_rss(&tab);
            if navigation.is_err() {
                let err_msg = navigation.unwrap_err();
                panic!("Failed navigation: {err_msg}")
            }

            // navigation.unwrap();
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

            let incidents = parse_rss(&rss_content, &config.categories);

            info!("Incidents: {:?}", incidents);
        }
        Err(_e) => panic!("Could not open browser tab."),
    }
}

fn navigate_to_rss(tab: &std::sync::Arc<headless_chrome::Tab>) -> Result<(), String> {
    tab.set_user_agent(USER_AGENT, None, None)
        .map_err(|e| format!("Could not set user agent due to: {}", e))?;

    tab.navigate_to(URL)
        .map_err(|_e| "Could not navigate to homepage.")?;

    let rss_href = tab
        .wait_for_xpath(XPATH)
        .map_err(|_| "Could not find xpath.")
        .and_then(|el| get_rss_href(el));

    if rss_href.is_ok() {
        tab.navigate_to(rss_href?.as_str())
            .map_err(|_e| "Could not navigate to the RSS link")?;
        // workaround to the rendering of the XML by chrome
        // let _ = tab.reload(true, None).unwrap();
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
