use crate::migrations::MigrationProcess;
use common::configuration::ServiceConfiguration;
use common::persistence::SORTED_INCIDENTS_KEY;
use common::Record;
use log::{error, info};
use redis::{cmd, ConnectionLike, RedisError};

// Search for keys "incidents:incident" and "incidents:sorted" and remove them.
// Also recreate the sorted set containing all the keys
#[derive(Default, Debug)]
pub struct RecreateSortedSet {
    failed_migrations: Vec<String>,
}

impl MigrationProcess for RecreateSortedSet {
    fn migrate(&mut self, _conn: &mut dyn ConnectionLike, _service_config: &ServiceConfiguration) {
        let keys_to_remove = vec!["incidents:incident", "incidents:sorted"];

        for key in keys_to_remove {
            let result: Result<i16, RedisError> = cmd("DEL").arg(key).query(_conn);
            match result {
                Ok(_) => {
                    info!("Found and deleted key {}", key);
                }
                Err(error) => {
                    self.failed_migrations.push(String::from(key));
                    error!("There was an error deleting: {}", error)
                }
            }
        }
    }
    fn migrate_key(&mut self, key: &str, redis_conn: &mut dyn ConnectionLike) {
        info!("KEY {}", key);

        let record_json: String = cmd("GET").arg(key).query(redis_conn).expect("Could not get the value.");
        let record: Record = match serde_json::from_str(&record_json) {
            Ok(record) => record,
            Err(e) => {
                self.failed_migrations.push(record_json);
                error!("Could not parse JSON: {}", e);
                return;
            }
        };

        let record_timestamp = record
            .date
            .and_hms_opt(0, 0, 0)
            .expect("Could not get timestamp.")
            .and_utc()
            .timestamp();

        let zadd_result: Result<u16, RedisError> = cmd("ZADD")
            .arg(SORTED_INCIDENTS_KEY)
            .arg(record_timestamp)
            .arg(key)
            .query(redis_conn);

        if zadd_result.is_err() {
            error!("Could NOT ZADD new key: {}", zadd_result.unwrap_err());
            self.failed_migrations.push(record_json);
        }
    }

    fn get_start_version(&self) -> u64 {
        2
    }

    fn get_description(&self) -> String {
        String::from("Recreate the sorted set")
    }

    fn print_results(&self) {
        info!("FINISHED RENAME FOR {}", self.get_start_version());
        info!("Failed RENAME FOR {:?}", self.failed_migrations);
    }
}

#[cfg(test)]
mod tests {}
