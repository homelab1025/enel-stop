pub use common::persistence::generate_redis_key;
use common::persistence::SORTED_INCIDENTS_KEY;
use common::Record;
use log::error;
use redis::aio::ConnectionLike;
use redis::{cmd, RedisError};
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

pub async fn store_record<T>(incident: &Record, conn: &mut T) -> Result<Result<i32, String>, Result<i32, String>>
where
    T: ConnectionLike + Send + Sync,
{
    let ser_inc = match serde_json::to_string(&incident) {
        Err(e) => return Err(Err(e.to_string())),
        Ok(ser_res) => ser_res,
    };

    let new_key = generate_redis_key(&incident.id);

    let redis_result: Result<String, RedisError> = cmd("SET").arg(&new_key).arg(ser_inc).query_async(conn).await;

    Ok(match redis_result {
        Err(e) => {
            error!("Could not store incident: {}", e);
            Err(e.to_string())
        }
        Ok(_rr) => {
            let timestamp: &i64 = &incident.date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();

            let new_elements: Result<i32, RedisError> = cmd("ZADD")
                .arg(SORTED_INCIDENTS_KEY)
                .arg(timestamp)
                .arg(&new_key)
                .query_async(conn)
                .await;

            match new_elements {
                Err(e) => {
                    error!("Could not store the key in timestamp sorted set: {}", e);
                    Err(e.to_string())
                }
                Ok(res) => Ok(res),
            }
        }
    })
}
