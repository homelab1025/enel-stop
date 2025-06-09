pub mod redis_store {
    use common::Record;
    use log::error;
    use redis::{Commands, RedisError};
    pub use common::persistence::generate_redis_key;
    use common::persistence::SORTED_INCIDENTS_KEY;

    pub fn store_record(incident: &Record, conn: &mut redis::Connection) -> Result<i32, String> {
        let ser_inc = match serde_json::to_string(&incident) {
            Err(e) => return Err(e.to_string()),
            Ok(ser_res) => ser_res,
        };

        let new_key = generate_redis_key(&incident.id);
        let redis_result: Result<String, RedisError> = conn.set(&new_key, ser_inc);

        match redis_result {
            Err(e) => {
                error!("Could not store incident: {}", e);
                Err(e.to_string())
            }
            Ok(_rr) => {
                let timestamp: &i64 = &incident.date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
                let new_elements: Result<i32, RedisError> = conn.zadd(SORTED_INCIDENTS_KEY, &new_key, timestamp);

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
