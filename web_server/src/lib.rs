use log::info;
use redis::cmd;

mod migration;

pub type MigrationFunction = Box<dyn FnMut(&String, &mut redis::Connection)>;
pub fn call_migration(mut migrations: Vec<MigrationFunction>, redis_conn: &mut redis::Connection) {
    migrations.iter_mut().for_each(|migration_function| {
        migrate_records(migration_function, redis_conn);
    })
}

/// Blocking function for migrating the records stored in redis to another structure.
fn migrate_records<M>(mut migration: M, redis_conn: &mut redis::Connection)
where
    M: FnMut(&String, &mut redis::Connection),
{
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
            info!("Wen thru all the keys.");
            break;
        }
        cursor = next_cursor.clone();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn call_migration() {
    }
}