use log::{error, info};
use rss::{Channel, Item};

pub fn parse_rss(rss_content: &str, filter_categs: &Vec<String>) -> Vec<Item> {
    info!("Filtering for categs: {:?}", filter_categs);

    let channel = match Channel::read_from(rss_content.as_bytes()) {
        Ok(channel) => channel,
        Err(err) => {
            error!("There was an error parsing the RSS: {}", err);
            return vec![];
        }
    };

    filter_incidents(&channel.items, &convert_config_categs(filter_categs))
}
