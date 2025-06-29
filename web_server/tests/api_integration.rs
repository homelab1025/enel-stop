mod common;

mod tests {
    use crate::common::setup_redis;
    use axum::extract::State;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use web_server::api::{AppState, RecordCount};

    #[tokio::test]
    async fn test_api_count() {
        let redis_client = setup_redis().await;

        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        let async_redis_conn = redis_client
            .get_multiplexed_tokio_connection()
            .await
            .expect("Async connection to Redis");

        let state = AppState {
            ping_msg: "The state of ping.".to_string(),
            redis_conn: Arc::new(Mutex::new(async_redis_conn)),
        };

        let resp = web_server::api::count_incidents(State(state)).await;
        assert!(resp.is_ok());

        let json: RecordCount = resp.expect("Should be OK").0;
        assert_eq!(3, json.total_count);

        // let container_port = testcontainers::core::ContainerPort::Tcp(6379);
        // let redis_container = GenericImage::new("redis", REDIS_TAG)
        //     .with_exposed_port(container_port)
        //     .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        //     .with_wait_for(WaitFor::seconds(10))
        //     .start()
        //     .await
        //     .unwrap();
        // let redis_host = redis_container.get_host().await.unwrap();
        // let redis_port = redis_container.get_host_port_ipv4(6379).await.unwrap();
        //
        // println!("Container: {}", redis_container.id());
        // println!("Redis HOST: {}", redis_host);
        // println!("Redis HOST PORT: {}", redis_port);
        // let conn_string = format!("redis://{}:{}/", redis_host, redis_port);
        // println!("Connection string: {}", conn_string);
        //
        // let redis_client = redis::Client::open(conn_string).expect("Could not connect to own container.");
        // let mut conn = redis_client.get_connection().expect("Could not create connection.");
        //
        // // create records and insert them
        // let incident1_county1 = Record {
        //     id: "test_id".to_string(),
        //     title: "test_title".to_string(),
        //     description: "test_description".to_string(),
        //     date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
        //     judet: "test_judet".to_string(),
        //     localitate: "test_localitate".to_string(),
        // };
        // let incident2_county1 = Record {
        //     id: "test_id2".to_string(),
        //     title: "test_title2".to_string(),
        //     description: "test_description2".to_string(),
        //     date: chrono::NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
        //     judet: "test_judet".to_string(),
        //     localitate: "test_localitate".to_string(),
        // };
        // let incident3_county2 = Record {
        //     id: "test_id3".to_string(),
        //     title: "test_title3".to_string(),
        //     description: "test_description3".to_string(),
        //     date: chrono::NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
        //     judet: "test_judet2".to_string(),
        //     localitate: "test_localitate".to_string(),
        // };
        //
        // let _res = store_record(&incident1_county1, &mut conn);
        // let _res = store_record(&incident2_county1, &mut conn);
        // let _res = store_record(&incident3_county2, &mut conn);
        //
        // let async_redis_conn = redis_client
        //     .get_multiplexed_async_connection()
        //     .await
        //     .expect("Could not ASYNC connect to Redis.");
        //
        // let state = AppState {
        //     ping_msg: "The state of ping.".to_string(),
        //     redis_conn: Arc::new(Mutex::new(async_redis_conn)),
        // };
        //
        // let resp = web_server::api::count_incidents(State(state)).await;
        // assert!(resp.is_ok());
        //
        // let json: RecordCount = resp.expect("Should be OK").0;
        // assert_eq!(3, json.total_count);
    }
}
