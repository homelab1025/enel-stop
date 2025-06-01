use crate::migrations::MigrationProcess;
use log::{error, info};
use redis::{cmd, ConnectionLike, RedisError};
use std::ops::DerefMut;

pub mod migrations;

const DB_VERSION_KEY: &str = "db_version";

pub fn call_migration(migrations: &mut Vec<&mut dyn MigrationProcess>, redis_conn: &mut dyn ConnectionLike) {
    let current_version: Result<u64, RedisError> = cmd("GET").arg(DB_VERSION_KEY).query(redis_conn);

    let current_version = match current_version {
        Ok(db_version) => db_version,
        Err(err) => panic!("{}", err),
    };

    migrations
        .iter_mut()
        .filter(|migration_function| migration_function.get_start_version() >= current_version)
        .for_each(|migration_function| {
            migrate_records(migration_function.deref_mut(), redis_conn);
        })
}

/// Blocking function for migrating the records stored in redis to another structure.
fn migrate_records(migration: &mut dyn MigrationProcess, redis_conn: &mut dyn ConnectionLike) {
    let mut cursor = String::from("0");
    loop {
        let (next_cursor, keys): (String, Vec<String>) = cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg("12*")
            .arg("COUNT")
            .arg("1000")
            .query(redis_conn)
            .expect("Could not run SCAN command");

        keys.iter().for_each(|key| {
            migration.migrate(key, redis_conn);
        });

        if next_cursor == "0" {
            info!("Went thru all the keys.");
            break;
        }
        cursor = next_cursor.clone();
    }

    // TODO: actually test the version incr
    // TODO: check that the migration has worked out fine and ONLY then increment the DB version
    let version_result: Result<u16, RedisError> = cmd("INCR").arg(DB_VERSION_KEY).query(redis_conn);
    match version_result {
        Ok(version) => {
            info!("New version: {}", version)
        }
        Err(err) => {
            error!("Could not increment version, but ran the migration function: {}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{call_migration, MigrationProcess, DB_VERSION_KEY};
    use redis::Value::SimpleString;
    use redis::{cmd, ConnectionLike, Value};
    use redis_test::{MockCmd, MockRedisConnection};

    #[test]
    fn call_migration_all() {
        let mut conn = MockRedisConnection::new(vec![
            MockCmd::new(cmd("GET").arg(DB_VERSION_KEY), Ok(Value::Int(0))),
            MockCmd::new(
                cmd("SCAN").arg("0").arg("MATCH").arg("12*").arg("COUNT").arg("1000"),
                Ok(Value::Array(vec![
                    SimpleString("0".to_string()),
                    Value::Array(vec![SimpleString("key1".to_string()), SimpleString("key2".to_string())]),
                ])),
            ),
            MockCmd::new(cmd("INCR").arg(DB_VERSION_KEY), Ok(Value::Int(1))),
            MockCmd::new(
                cmd("SCAN").arg("0").arg("MATCH").arg("12*").arg("COUNT").arg("1000"),
                Ok(Value::Array(vec![
                    SimpleString("0".to_string()),
                    Value::Array(vec![SimpleString("key1".to_string()), SimpleString("key2".to_string())]),
                ])),
            ),
            MockCmd::new(cmd("INCR").arg(DB_VERSION_KEY), Ok(Value::Int(2))),
        ]);

        #[derive(Default)]
        struct MockMigration1 {
            key1_counter: i32,
            key2_counter: i32,
        }

        impl MigrationProcess for MockMigration1 {
            fn migrate(&mut self, key: &str, _conn: &mut dyn ConnectionLike) {
                match key {
                    "key1" => {
                        println!("Key 1 hit.");
                        self.key1_counter += 1;
                    }
                    "key2" => {
                        println!("Key 2 hit.");
                        self.key2_counter += 1;
                    }
                    _ => {}
                }
            }

            fn get_start_version(&self) -> u64 {
                0
            }

            fn get_description(&self) -> String {
                "MockMigration1".to_string()
            }
        }
        impl MockMigration1 {
            fn get_key1_counter(&self) -> i32 {
                self.key1_counter
            }
            fn get_key2_counter(&self) -> i32 {
                self.key2_counter
            }
        }

        #[derive(Default)]
        struct MockMigration2 {
            key1_counter: i32,
            key2_counter: i32,
        }
        impl MigrationProcess for MockMigration2 {
            fn migrate(&mut self, key: &str, _conn: &mut dyn ConnectionLike) {
                match key {
                    "key1" => {
                        println!("Key 1 hit.");
                        self.key1_counter += 1;
                    }
                    "key2" => {
                        println!("Key 2 hit.");
                        self.key2_counter += 1;
                    }
                    _ => {}
                }
            }
            fn get_start_version(&self) -> u64 {
                1
            }

            fn get_description(&self) -> String {
                "MockMigration2".to_string()
            }
        }
        impl MockMigration2 {
            fn get_key1_counter(&self) -> i32 {
                self.key1_counter
            }
            fn get_key2_counter(&self) -> i32 {
                self.key2_counter
            }
        }

        let mut m1: MockMigration1 = Default::default();
        let mut m2: MockMigration2 = Default::default();
        let mut migrations: Vec<&mut dyn MigrationProcess> = vec![&mut m1, &mut m2];

        call_migration(&mut migrations, &mut conn);

        assert_eq!(1, m1.get_key1_counter());
        assert_eq!(1, m1.get_key2_counter());
        assert_eq!(1, m2.get_key1_counter());
        assert_eq!(1, m2.get_key2_counter());
    }
}
