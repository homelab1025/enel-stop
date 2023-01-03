use std::collections::HashMap;
use std::env;

use rss::Channel;

use crate::param_validation::{ArgumentsError, validate_params};

mod param_validation;

/**
URL: https://www.e-distributie.com/content/dam/e-distributie/outages/rss/enel_rss_muntenia.xml
 */
fn main() -> Result<(), ArgumentsError> {
    let cli_arg = env::args().nth(1);
    let url = validate_params(cli_arg)?;

    println!("Input is valid. Starting to parse it.");

    let content = reqwest::blocking::get(url)
        .unwrap()
        .bytes()
        .unwrap();

    let channel = Channel::read_from(&content[..]).unwrap();

    let affected_locations = channel.items.len();
    println!("Scheduled downtime locations: {}", affected_locations);

    if affected_locations > 10 {
        send_sms(affected_locations);
    }


    Ok(())
}

fn send_sms(locations_counter: usize) {
    todo!()
}