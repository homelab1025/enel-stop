use common::Record;
use log::error;
use sqlx::{Pool, Postgres};
use std::ops::Deref;
use std::sync::Arc;

const INSERT_QUERY: &str = "INSERT INTO incidents(external_id, day, county, location, description) \
 VALUES ($1, $2, $3, $4, $5) \
 ON CONFLICT (external_id) DO \
 UPDATE SET day = $2, county = $3, location = $4, description = $5";

pub async fn new_store_record(record: &Record, pg_pool: Arc<Pool<Postgres>>) -> Result<u64, String> {
    let pg_incident = sqlx::query(INSERT_QUERY)
        .bind(&record.id)
        .bind(&record.date)
        .bind(&record.county)
        .bind(&record.location)
        .bind(&record.description)
        .execute(pg_pool.deref())
        .await;

    match pg_incident {
        Ok(incident) => Ok(incident.rows_affected()),
        Err(e) => {
            error!("Could not store record as incident: {}", e);
            Err(e.to_string())
        }
    }
}
