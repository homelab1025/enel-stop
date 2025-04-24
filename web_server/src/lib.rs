use log::info;
use redis::{cmd, ConnectionLike, RedisResult, Value};

mod migration;

pub type MigrationFunction = dyn FnMut(&String, &mut dyn ConnectionLike) -> ();

pub trait MigrationF {
    fn migrate(&mut self, key: &str, _conn: &mut dyn ConnectionLike);
}

pub fn call_migration(migrations: &mut Vec<Box<dyn MigrationF>>, redis_conn: &mut dyn ConnectionLike) {
    migrations.iter_mut().for_each(|migration_function| {
        migrate_records(migration_function, redis_conn);
    })
}

/// Blocking function for migrating the records stored in redis to another structure.
fn migrate_records(migration: &mut Box<dyn MigrationF>, redis_conn: &mut dyn ConnectionLike) {
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
            // migration(key, redis_conn);
        });

        if next_cursor == "0" {
            info!("Went thru all the keys.");
            break;
        }
        cursor = next_cursor.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::{call_migration, MigrationF};
    use redis::Value::SimpleString;
    use redis::{ConnectionLike, RedisResult, Value};
    use std::cell::RefCell;

    struct MockedConnection {}
    impl ConnectionLike for MockedConnection {
        fn req_packed_command(&mut self, cmd: &[u8]) -> RedisResult<Value> {
            Ok(Value::Array(vec![
                SimpleString("0".to_string()),
                Value::Array(vec![SimpleString("key1".to_string()), SimpleString("key2".to_string())]),
            ]))
        }

        fn req_packed_commands(&mut self, cmd: &[u8], offset: usize, count: usize) -> RedisResult<Vec<Value>> {
            Ok(vec![SimpleString("key1".to_string()), SimpleString("key2".to_string())])
        }

        fn get_db(&self) -> i64 {
            64
        }

        fn check_connection(&mut self) -> bool {
            true
        }

        fn is_open(&self) -> bool {
            true
        }
    }

    #[test]
    fn call_migration_all() {
        let mut conn = MockedConnection {};

        #[derive(Default)]
        struct M1 {
            key1_counter: i32,
            key2_counter: i32,
        };
        impl MigrationF for M1 {
            fn migrate(&mut self, key: &str, _conn: &mut dyn ConnectionLike) {
                match key {
                    "key1" => {
                        println!("Key 1 hit.");
                        self.key1_counter += 1;
                    }
                    "key2" => {
                        println!("Key 2 hit.");
                        self.key1_counter += 1;
                    }
                    _ => {}
                }
            }
        }
        impl M1 {
            fn get_key1_counter(&self) -> i32 {
                self.key1_counter
            }
            fn get_key2_counter(&self) -> i32 {
                self.key2_counter
            }
        }

        let mut m1: M1 = Default::default();
        let mut m2: M1 = Default::default();

        let mut migrations: Vec<Box<dyn MigrationF>> = vec![Box::new(m1), Box::new(m2)];

        call_migration(&mut migrations, &mut conn);

        // println!("{}", m1.get_key1_counter());
    }
}
