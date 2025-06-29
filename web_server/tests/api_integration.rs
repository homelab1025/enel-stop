mod common;

mod tests {
    use crate::common::setup_redis;
    use axum::extract::State;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use web_server::api::{AppState, RecordCount};

    #[tokio::test]
    async fn test_api_count() {
        let (redis_client, _redis_container) = setup_redis().await;

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
    }
}
