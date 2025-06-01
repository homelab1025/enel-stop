use common::Record;
use log::{debug, error};
use redis::{cmd, ConnectionLike, RedisError};
use crate::migrations::MigrationProcess;

/// CURRENT: The incident is serialized to json and the json string is mapped to a key which is the guid of the RSS element.
/// NEXT: Add the incident ID in a sorted set and use the timestamp as score. Create a new entry and remove the old one.
/// The new entry should have a prefix "incident:".
#[derive(Default, Debug)]
pub struct SortedSetMigration {
    failed_migrations: Vec<String>,
    recycle_bin: Vec<String>,
}

impl MigrationProcess for SortedSetMigration {
    fn migrate(&mut self, key: &str, redis_conn: &mut dyn ConnectionLike) {
        debug!("KEY {}", key);

        let record_json: String = cmd("GET").arg(key).query(redis_conn).expect("Could not get the value.");
        let record: Record = match serde_json::from_str(&record_json) {
            Ok(record) => record,
            Err(e) => {
                self.failed_migrations.push(record_json);
                error!("Could not parse JSON: {}", e);
                return;
            }
        };

        // let record_timestamp = record.date.and_hms(0, 0, 0).and_utc().timestamp();
        let record_timestamp = record
            .date
            .and_hms_opt(0, 0, 0)
            .expect("Could not get timestamp.")
            .and_utc()
            .timestamp();

        debug!(
            "Got key {} with time {} so timestamp is {}",
            record.id, record.date, record_timestamp
        );

        let new_key = format!("incident:{}", record.id);
        let new_key_result: Result<String, RedisError> = cmd("SET")
            .arg(new_key.clone())
            .arg(record_json.clone())
            .query(redis_conn);
        if new_key_result.is_err() {
            error!("Could NOT store new key: {}", new_key_result.unwrap_err());
            self.failed_migrations.push(record_json);
            return;
        }

        let zadd_result: Result<u16, RedisError> = cmd("ZADD")
            .arg("incidents:sorted")
            .arg(record_timestamp)
            .arg(new_key)
            .query(redis_conn);
        if zadd_result.is_err() {
            error!("Could NOT ZADD new key: {}", zadd_result.unwrap_err());
            self.failed_migrations.push(record_json);
            return;
        }
        
        let del_result: Result<u16, RedisError> = cmd("DEL").arg(record.id).query(redis_conn);
        if del_result.is_err() {
            error!("Could NOT delete key: {}", del_result.unwrap_err());
            self.recycle_bin.push(record_json);
        }
    }
fn get_start_version(&self) -> u64 {
    0
}

    fn get_description(&self) -> String {
        String::from("CURRENT: The incident is serialized to json and the json string is mapped to a key which is the guid of the RSS element.\nNEXT: Add the incident ID in a sorted set and use the timestamp as score. Create a new entry and remove the old one.")
    }
}

#[cfg(test)]
mod tests {
    use common::Record;
    use redis::{cmd, Value};
    use redis_test::{MockCmd, MockRedisConnection};
    use crate::migrations::MigrationProcess;
    use crate::migrations::sorted_set::SortedSetMigration;

    #[test]
    fn test_migrate_happy_path() {
        let record = Record {
            description: String::from("description"),
            id: String::from("test-id"),
            judet: String::from("judet"),
            localitate: String::from("localitate"),
            title: String::from("title"),
            date: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Could not get date."),
        };

        let old_json = serde_json::to_string(&record).unwrap();

        let commands = vec![
            MockCmd::new(
                redis::cmd("GET").arg("test-id"),
                Ok(Value::SimpleString(old_json.clone())),
            ),
            MockCmd::new(
                cmd("SET").arg("incident:test-id").arg(old_json),
                Ok(Value::SimpleString("OK".to_string())),
            ),
            MockCmd::new(
                cmd("ZADD")
                    .arg("incidents:sorted")
                    .arg("1577836800")
                    .arg("incident:test-id"),
                Ok(Value::Int(1)),
            ),
            MockCmd::new(
                cmd("DEL")
                    .arg("test-id"),
                Ok(Value::Int(1)),
            ),
        ];
        let mut mocked_conn = MockRedisConnection::new(commands);

        let mut migration = SortedSetMigration::default();
        migration.migrate("test-id", &mut mocked_conn);

        println!("Failed migrations: {:?}", migration.failed_migrations);
        assert_eq!(
            0,
            migration.failed_migrations.len(),
            "There should be no failed migrations."
        );

        println!("Lingering records (migrated, but not removed): {:?}", migration.recycle_bin);
        assert_eq!(
            0,
            migration.recycle_bin.len(),
            "There should be no lingering records."
        );
    }
}
