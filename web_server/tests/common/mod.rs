use common::Record;
use redis::Client;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use testcontainers::core::WaitFor;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage};
use tokio::sync::Mutex;
use web_server::AppState;
use web_server::scraper::redis_store::store_record;

pub const REDIS_TAG: &str = "7.4.2";
pub const FILTERING_COUNTY: &str = "test_judet";

pub async fn setup_redis() -> (Client, ContainerAsync<GenericImage>) {
    let container_port = testcontainers::core::ContainerPort::Tcp(6379);
    let redis_container = GenericImage::new("redis", REDIS_TAG)
        .with_exposed_port(container_port)
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections tcp"))
        .with_wait_for(WaitFor::seconds(5))
        .start()
        .await
        .unwrap();

    let redis_host = redis_container.get_host().await.unwrap();
    let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();

    println!("Container: {}", redis_container.id());
    println!("Redis HOST: {}", redis_host);
    println!("Redis HOST PORT: {}", redis_port);
    let conn_string = format!("redis://{}:{}/", redis_host, redis_port);
    println!("Connection string: {}", conn_string);

    let redis_client = Client::open(conn_string).expect("Connecting to the redis container");
    let mut conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Creating sync connection to redis");

    // create records and insert them

    let incident1_county1 = Record {
        id: "test_id".to_string(),
        title: "test_title".to_string(),
        description: "test_description".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
        county: FILTERING_COUNTY.to_string(),
        location: "test_localitate".to_string(),
    };
    let incident2_county1 = Record {
        id: "test_id2".to_string(),
        title: "test_title2".to_string(),
        description: "test_description2".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
        county: FILTERING_COUNTY.to_string(),
        location: "test_localitate".to_string(),
    };
    let incident3_county2 = Record {
        id: "test_id3".to_string(),
        title: "test_title3".to_string(),
        description: "test_description3".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
        county: "test_judet2".to_string(),
        location: "test_localitate2".to_string(),
    };
    let incident4_county2 = Record {
        id: "test_id4".to_string(),
        title: "test_title3".to_string(),
        description: "test_description3".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
        county: "test_judet2".to_string(),
        location: "test_localitate2".to_string(),
    };
    let incident5_county2 = Record {
        id: "test_id5".to_string(),
        title: "test_title3".to_string(),
        description: "test_description3".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
        county: "test_judet2".to_string(),
        location: "test_localitate2".to_string(),
    };

    let _res = store_record(&incident1_county1, &mut conn).await;
    let _res = store_record(&incident2_county1, &mut conn).await;
    let _res = store_record(&incident3_county2, &mut conn).await;
    let _res = store_record(&incident4_county2, &mut conn).await;
    let _res = store_record(&incident5_county2, &mut conn).await;

    (redis_client, redis_container)
}

pub async fn setup_app_state() -> (AppState<MultiplexedConnection>, ContainerAsync<GenericImage>) {
    let (redis_client, redis_container) = setup_redis().await;

    let async_redis_conn = redis_client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Async connection to Redis");

    let state = AppState {
        ping_msg: "The state of ping.".to_string(),
        redis_conn: Arc::new(Mutex::new(async_redis_conn)),
        categories: vec![],
        metrics: Default::default(),
    };

    (state, redis_container)
}
