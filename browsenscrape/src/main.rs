use core::panic;

use headless_chrome::Browser;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let url = "https://www.reteleelectrice.ro/intreruperi/programate/";

    let browser_result = Browser::default();
    match browser_result {
        Ok(browser) => {
            let browser_tab = browser.new_tab();
            if browser_tab.is_err() {
                panic!("Can not open browser tab.")
            }

            let browser_tab = browser_tab.unwrap();
            let navigation_result = browser_tab.navigate_to(url);
            if navigation_result.is_err() {
                panic!("Could not navigate to {}", url);
            }
        }
        Err(err) => {
            panic!("Can not instantiate browser: {}", err);
        }
    }
}
