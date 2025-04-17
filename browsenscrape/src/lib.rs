pub mod redis_store {
    use log::error;
    use redis::{Commands, RedisError};

    pub fn store_record(incident: &common::Record, conn: &mut redis::Connection) -> Result<i32, String> {
        let ser_inc = match serde_json::to_string(&incident) {
            Err(e) => return Err(e.to_string()),
            Ok(ser_res) => ser_res,
        };

        let redis_result: Result<String, RedisError> = conn.set(&incident.id, ser_inc);

        match redis_result {
            Err(e) => {
                error!("Could not store incident: {}", e);
                Err(e.to_string())
            }
            Ok(_rr) => {
                let timestamp: &i64 = &incident.date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
                let new_elements: Result<i32, RedisError> = conn.zadd("incidents:sorted", &incident.id, timestamp);

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
