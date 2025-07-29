use chrono::Utc;

use crate::common::{generate_ddl, setup_logging};
use ::common::Record;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres;
use tokio::time::sleep;
use web_server::scraper::persistence::new_store_record;
use web_server::web_api::Incident;

mod common;
#[tokio::test]
async fn test_persistence() {
    setup_logging();

    let pg = postgres::Postgres::default()
        .with_user("postgres")
        .with_password("postgres")
        .with_db_name("enel")
        .start()
        .await
        .unwrap();
    let pg_port = pg.get_host_port_ipv4(5432).await.unwrap();
    let pg_host = pg.get_host().await.unwrap();

    sleep(Duration::from_secs(5)).await;

    let pg_conn_string = format!("postgres://postgres:postgres@{}:{}/enel", &pg_host, &pg_port);
    info!("Connecting to postgres: {}", &pg_conn_string);
    let pg_pool = Arc::new(PgPoolOptions::new().connect(&pg_conn_string).await.unwrap());

    let ddl = generate_ddl().unwrap();
    let _res = sqlx::raw_sql(ddl.as_ref()).execute(pg_pool.deref()).await.unwrap();

    let current_day = Utc::now().date_naive();
    let record = Record {
        location: String::from("location"),
        county: String::from("county"),
        id: String::from("666id"),
        description: String::from("descr"),
        title: String::from("title"),
        date: current_day,
    };

    let _res = new_store_record(&record, pg_pool.clone()).await.unwrap();

    let incident: Incident = sqlx::query_as("SELECT * FROM incidents WHERE external_id = $1")
        .bind(&record.id)
        .fetch_one(pg_pool.deref())
        .await
        .unwrap();

    assert_eq!(record.id, incident.external_id);
    assert_eq!(record.date, incident.day);
    assert_eq!(record.description, incident.description);
    assert_eq!(current_day, incident.day);
    assert_eq!(record.county, incident.county);
    assert_eq!(record.location, incident.location);
}
