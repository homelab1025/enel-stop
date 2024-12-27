use core::panic;
use std::{env, ffi::OsStr};

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
        panic!("No configuration has been provided.")
    }

    let config = config.unwrap();

    if config.is_err() {
        panic!("some other config issue");
    }
    let config = config.unwrap();

    let browser_result = Browser::new(
        LaunchOptionsBuilder::default()
            .enable_logging(true)
            // .args(vec![OsStr::new("--single-process")])
            .build()
            .unwrap(),
    );

    info!("right after init");
    if browser_result.is_err() {
        panic!("Could not get browser.")
    }
    let browser_tab = browser_result.unwrap().new_tab();

    info!("right after unwrap");
    match browser_tab {
        Ok(tab) => {
            info!("right before navigation start");
            let navigation = navigate_to_rss(&tab);
            if navigation.is_err() {
                let err_msg = navigation.unwrap_err();
                panic!("Failed navigation: {err_msg}")
            }

            let _content = tab
                .get_content()
                .map_err(|_| "Could not get content")
                .map(|c| parse_rss(&c, &config.categories));
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
    } else {
        return Err("Could not extrat the rss href".to_string());
    }
    Ok(())
}

// fn extract_items(rss_content: String, filter_categs: &Vec<String>) -> Vec<Record> {
//     let all_items = rss_reader::parse_rss(&rss_content, filter_categs);
//     // let filtered_items = filter_items(all_items, filter_categs);
//     convert_items(all_items)
// }

// fn convert_items(all_items: Vec<rss::Item>) -> Vec<Record> {
//     let location_extract_pattern = r"(.*?) Judet: (\w+)\s+Localitate: (.+)";
//     let location_extractor = Regex::new(location_extract_pattern).unwrap();
//
//     all_items
//         .iter()
//         // .filter(|item| true)
//         .filter_map(|item| {
//             let title = item.title.as_ref()?;
//
//             location_extractor.captures(title).and_then(|capture| {
//                 let judet = capture.get(2).unwrap().as_str();
//                 let localitate = capture.get(3)?.as_str();
//                 let id = item.guid.as_ref()?;
//
//                 Option::Some(Record {
//                     id: id.value.to_string(),
//                     judet: judet.to_string(),
//                     localitate: localitate.to_string(),
//                     title: item.title.as_ref()?.to_string(),
//                     description: item.description.as_ref()?.to_string(),
//                 })
//             })
//         })
//         .collect::<Vec<_>>()
// }

fn get_rss_href(extracted_element: headless_chrome::Element<'_>) -> Result<String, &'static str> {
    match extracted_element.get_attribute_value("href") {
        Ok(Some(href)) => Ok(href),
        Ok(None) => Err("No href was set."),
        Err(_error) => Err("could not extract the element attribute href"),
    }
}
