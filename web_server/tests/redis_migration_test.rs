use common::Record;
use testcontainers::core::WaitFor;
use testcontainers::runners::SyncRunner;
use testcontainers::GenericImage;
use web_server::call_migration;

const REDIS_TAG: &str = "7.4.2";

#[test]
fn test_redis_storage_migration() {
    let container_port = testcontainers::core::ContainerPort::Tcp(6379);
    let redis_container = GenericImage::new("redis", REDIS_TAG)
        .with_exposed_port(container_port)
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_wait_for(WaitFor::seconds(5))
        .start()
        .unwrap();
    let redis_host = redis_container.get_host().unwrap();
    let redis_port = redis_container.get_host_port_ipv4(6379).unwrap();

    println!("Container: {}", redis_container.id());
    println!("Redis HOST: {}", redis_host);
    println!("Redis HOST PORT: {}", redis_port);
    let conn_string = format!("redis://{}:{}/", redis_host, redis_port);
    println!("Connection string: {}", conn_string);

    let mut conn = redis::Client::open(conn_string)
        .expect("Could not connect to own container.")
        .get_connection()
        .expect("Could not create connection.");

    // call_migration(&mut conn);

    let incident = Record {
        id: "test_id".to_string(),
        title: "test_title".to_string(),
        description: "test_description".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
        judet: "test_judet".to_string(),
        localitate: "test_localitate".to_string(),
    };
}
