pub mod redis_store {
    pub use common::persistence::generate_redis_key;
    use common::persistence::SORTED_INCIDENTS_KEY;
    use common::Record;
    use log::error;
    use redis::{cmd, RedisError};

    pub fn store_record(incident: &Record, conn: &mut dyn redis::ConnectionLike) -> Result<i32, String> {
        let ser_inc = match serde_json::to_string(&incident) {
            Err(e) => return Err(e.to_string()),
            Ok(ser_res) => ser_res,
        };

        let new_key = generate_redis_key(&incident.id);

        let redis_result: Result<String, RedisError> = cmd("SET").arg(&new_key).arg(ser_inc).query(conn);

        match redis_result {
            Err(e) => {
                error!("Could not store incident: {}", e);
                Err(e.to_string())
            }
            Ok(_rr) => {
                let timestamp: &i64 = &incident.date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();

                let new_elements: Result<i32, RedisError> = cmd("ZADD")
                    .arg(SORTED_INCIDENTS_KEY)
                    .arg(&new_key)
                    .arg(timestamp)
                    .query(conn);

                match new_elements {
                    Err(e) => {
                        error!("Could not store the key in timestamp sorted set: {}", e);
                        Err(e.to_string())
                    }
                    Ok(res) => Ok(res),
                }
            }
        }
    }
}
