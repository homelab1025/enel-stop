use browsenscrape::redis_store::store_record;
use common::configuration;
use core::panic;
use log::{debug, error, info, warn, LevelFilter};
use prometheus_client::{
    encoding::text::encode,
    metrics::{counter::Counter, gauge::Gauge},
    registry::Registry,
};
use rss_reader::parse_rss;
use simple_logger::SimpleLogger;
use std::{env, time::Instant};

mod rss_reader;

fn main() {
    let start_time_program = Instant::now();
    SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Debug)
        .with_colors(true)
        .init()
        .unwrap();

    let mut metrics_registry =
        Registry::with_prefix_and_labels("enel", [("app".into(), "browsenscrape".into())].into_iter());
    let gauge_full: Gauge = Gauge::default();
    let incidents_count: Counter = Counter::default();
    let failures_count: Counter = Counter::default();

    metrics_registry.register(
        "incidents_count",
        "Number of stored incidents.",
        incidents_count.clone(),
    );
    metrics_registry.register("full", "Time it takes to run the whole cron job.", gauge_full.clone());

    metrics_registry.register(
        "failures",
        "Failures: most probably due to blocking by WAF.",
        failures_count.clone(),
    );

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

    let rss_content = get_rss_content(&config.url);
    match rss_content {
        Ok(content) => {
            let incidents = parse_rss(&content, &config.categories);
            if incidents.is_empty() {
                failures_count.inc();
            }

            debug!("Incidents: {:?}", incidents);

            if let Some(conn) = redis_connection.as_mut() {
                let stored_incidents = incidents
                    .iter()
                    .map(|incident| store_record(incident, conn))
                    .filter(|incident_result| incident_result.is_ok())
                    .count();

                incidents_count.inc_by(stored_incidents.try_into().unwrap());

                info!(
                    "Stored {} incidents out of {} received.",
                    stored_incidents,
                    incidents.len()
                );
            }
        }
        Err(err) => {
            error!("{}", err);
            failures_count.inc();
        }
    }

    gauge_full.set(start_time_program.elapsed().as_millis().try_into().unwrap());
    config.pushgateway_server.map(|pushgw_server| {
        let metrics_push = push_metrics(&metrics_registry, &pushgw_server);

        match metrics_push {
            Ok(()) => info!("Pushed metrics to prometheus gateway."),
            Err(err) => warn!("Could not push metrics to prometheus gateway: {}", err),
        }
    });
}

fn push_metrics(metrics_registry: &Registry, pushgateway: &str) -> Result<(), String> {
    let mut buffer = String::new();
    encode(&mut buffer, metrics_registry).map_err(|e| e.to_string())?;

    println!("Metrics: {}", &buffer);

    let http_client = reqwest::blocking::ClientBuilder::new()
        .build()
        .map_err(|e| e.to_string())?;

    http_client
        .post(pushgateway)
        .body(buffer)
        .send()
        .map_err(|e| e.to_string())?;

    Ok(())
}

const USER_AGENT_HEADER_VALUE: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36";
const REFERER_HEADER_VALUE: &str = "https://www.reteleelectrice.ro/intreruperi/programate/";
const ACCEPT_HEADER_VALUE: &str = "application/rss+xml, application/xml, text/xml, */*";

fn get_rss_content(starting_url: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(starting_url)
        .header("User-Agent", USER_AGENT_HEADER_VALUE)
        .header("Referer", REFERER_HEADER_VALUE)
        .header("Accept", ACCEPT_HEADER_VALUE)
        .send()
        .map_err(|e| format!("Could not fetch rss content: {}", e))?;

    match response.text() {
        Ok(content) => Ok(content),
        Err(err) => Err(format!("Could not decode text from response: {}", err)),
    }
}
