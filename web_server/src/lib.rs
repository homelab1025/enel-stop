use log::info;
use redis::{cmd, ConnectionLike, RedisResult, Value};

mod migration;

pub type MigrationFunction = Box<dyn FnMut(&String, &mut dyn ConnectionLike)>;

pub fn call_migration(migrations: &mut Vec<MigrationFunction>, redis_conn: &mut dyn ConnectionLike) {
    migrations.iter_mut().for_each(|migration_function| {
        migrate_records(migration_function, redis_conn);
    })
}

/// Blocking function for migrating the records stored in redis to another structure.
fn migrate_records(migration: &mut MigrationFunction, redis_conn: &mut dyn ConnectionLike) {
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
            migration(key, redis_conn);
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
    use std::cell::RefCell;
    use crate::{call_migration, MigrationFunction};
    use redis::Value::SimpleString;
    use redis::{ConnectionLike, RedisResult, Value};

    struct MockedConnection {
    }
    impl ConnectionLike for MockedConnection {
        fn req_packed_command(&mut self, cmd: &[u8]) -> RedisResult<Value> {
            Ok(Value::Array(vec![SimpleString("0".to_string()),Value::Array(vec![SimpleString("key1".to_string()), SimpleString("key2".to_string())])]))
        }

        fn req_packed_commands(&mut self, cmd: &[u8], offset: usize, count: usize) -> RedisResult<Vec<Value>> {
            Ok(vec![SimpleString("key1".to_string()),SimpleString("key2".to_string())])
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
        let mut conn = MockedConnection{};
        let key1_counter=  RefCell::new(0);

        let migration1:MigrationFunction = Box::new(|key: &String, _conn: &mut dyn ConnectionLike| {
            println!("Migrating #1 {}", key);
            // *key1_counter.borrow_mut() = 123;
        });
        let migration2:MigrationFunction = Box::new(|key: &String, _conn: &mut dyn ConnectionLike| {
            println!("Migrating #2 {}", key);
            // *key1_counter.borrow_mut() = 5432;
        });

        let mut migrations = vec![migration1, migration2];
        call_migration(&mut migrations, &mut conn);
        println!("{}", key1_counter.borrow());
    }
}
