use std::env;
use std::fs::File;

use rss::Channel;
use url::Url;

fn main() -> Result<(), String> {
    let url_arg = env::args().nth(1);

    let result = validate_url_arg(url_arg);

    // result.unwrap_or_else(|e| {
    //     panic!("!!!!");
    // });

    match result.is_ok() {
        true => {
            println!("Input is valid. Starting to parse it.");

            let content = reqwest::blocking::get(result.unwrap())
                .unwrap()
                .bytes()
                .unwrap();

            let channel = Channel::read_from(&content[..]).unwrap();

            // for item in channel.items {
            //     item.
            // }

            println!("items: {:?}", channel.items);

            Ok(())
        }
        false => Err(result.err().unwrap())
    }

    // if result.is_ok() {
    //     println!("Input is valid. Starting to parse it.");
    //     Ok(())
    // } else {
    //     result
    // }

    // if result.is_err() {
    //     result
    // }
    //
    // println!("Input is valid. Starting to parse it.");
    //
    // Ok(())

    // if rss_url.is_err() {
    //     // panic!("The provided RSS URL is not valid");
    //     return Err(rss_url.err().unwrap());
    // }

    //
    // let rss_path = args.get(0).unwrap_or_else(|| {
    //     println!("no rss path for the program");
    //     std::process::exit(-1);
    // });
    //
    // let rss_path = "https://www.e-distributie.com/content/dam/e-distributie/outages/rss/enel_rss_muntenia.xml";
    // let rss_path = String::from("https://www.e-distributie.com/content/dam/e-distributie/outages/rss/enel_rss_muntenia.xml");
    //
    // println!("Scanning the RSS at the following address: ");
}

fn validate_url_arg(cli_arg: Option<String>) -> Result<Url, String> {
    if let Some(url_path) = cli_arg {
        let rss_url = Url::parse(&url_path);

        match rss_url.is_err() {
            true => Err(String::from("The provided URL is not valid.")),
            false => {
                let url = rss_url.unwrap();
                println!("Using URL: {}", url);
                Ok(url)
            }
        }
    } else {
        Err(String::from("No RSS URL has been provided."))
    }
}
