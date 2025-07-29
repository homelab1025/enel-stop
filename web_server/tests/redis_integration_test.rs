use common::persistence::generate_redis_key;
use common::Record;
use redis::cmd;
use testcontainers::core::WaitFor;
use testcontainers::runners::AsyncRunner;
use testcontainers::GenericImage;
use web_server::scraper::persistence::store_record;

const REDIS_TAG: &str = "7.4.2";

#[tokio::test]
async fn test_redis_storage() {
    let container_port = testcontainers::core::ContainerPort::Tcp(6379);
    let redis_container = GenericImage::new("redis", REDIS_TAG)
        .with_exposed_port(container_port)
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
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

    let mut conn = redis::Client::open(conn_string)
        .expect("Could not connect to own container.")
        .get_multiplexed_async_connection()
        .await
        .expect("Could not ASYNC connect to Redis.");

    let incident = Record {
        id: "test_id".to_string(),
        title: "test_title".to_string(),
        description: "test_description".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
        county: "test_judet".to_string(),
        location: "test_localitate".to_string(),
    };

    let _res = store_record(&incident, &mut conn, ).await;

    let redis_key = generate_redis_key("test_id");
    // let record = conn.get::<String, String>(redis_key.to_string()).unwrap();
    let record: String = cmd("GET").arg(redis_key).query_async(&mut conn).await.unwrap();
    assert_eq!(
        record,
        "{\"id\":\"test_id\",\"date\":\"2023-10-01\",\"county\":\"test_judet\",\"location\":\"test_localitate\",\"title\":\"test_title\",\"description\":\"test_description\"}"
    );
}
