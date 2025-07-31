use std::ops::Deref;
use crate::common::{TestInfrastructure, create_app_state};
use axum::extract::State;
use sqlx::Error;

mod common;

#[tokio::test]
async fn test_scraper_api() {
    let infra = TestInfrastructure::new().await;
    let state = create_app_state(&infra).await;

    let body: String = read_rss_file("tests/rss-outages-test.xml").await;
    let resp = web_server::scraper::scraper_api::submit_rss(State(state.clone()), body).await;
    assert!(resp.is_ok());

    let res: Result<i64, Error> = sqlx::query_scalar("SELECT COUNT(*) FROM incidents")
        .fetch_one(state.pg_pool.deref())
        .await;

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 9);
}

async fn read_rss_file(file_path: &str) -> String {
    String::from_utf8(tokio::fs::read(file_path).await.unwrap()).unwrap()
}
