use crate::migrations::MigrationProcess;
use log::{error, info};
use redis::{cmd, ConnectionLike, RedisError};

/// CURRENT: The key is of the form: "incident:<rss_key>"
/// NEXT: The key is of the form: "incidents:<rss_key>"
#[derive(Default)]
pub struct RenamePrefixMigration {
    failed_migrations: Vec<String>,
    skipped: Vec<String>,
}

impl MigrationProcess for RenamePrefixMigration {
    fn migrate(&mut self, key: &str, redis_conn: &mut dyn ConnectionLike) {
        info!("KEY {}", key);

        let key_components: Vec<&str> = key.split(':').collect();
        let new_key = format!("incidents:{}", key_components[0]);
        if key_components.len() == 2 && key_components[0] == "incident" {
            let result: Result<String, RedisError> = cmd("RENAME").arg(key).arg(new_key).query(redis_conn);
            match result {
                Ok(valid) => {
                    info!("Renamed key {} to {}", key, valid);
                }
                Err(error) => {
                    self.failed_migrations.push(String::from(key));
                    error!("There was an error renaming: {}", error)
                }
            }
        } else {
            info!("Skipping key {}", key);
            self.skipped.push(String::from(key));
        }
    }

    fn get_start_version(&self) -> u64 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::migrations::rename_prefix::RenamePrefixMigration;
    use crate::migrations::MigrationProcess;
    use redis::{cmd, Value};
    use redis_test::{MockCmd, MockRedisConnection};

    #[test]
    fn test_migrate_renaming() {
        let commands = vec![MockCmd::new(
            cmd("RENAME").arg("incident:123").arg("incidents:123"),
            Ok(Value::SimpleString(String::from("OK"))),
        )];
        let mut mocked_conn = MockRedisConnection::new(commands);

        let mut migration = RenamePrefixMigration::default();
        migration.migrate("incident:123", &mut mocked_conn);

        assert_eq!(0, migration.skipped.len());
    }
}
