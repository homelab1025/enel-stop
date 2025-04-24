use common::Record;
use log::info;
use redis::{cmd, ConnectionLike};

pub trait MigrationProcess {
    fn migrate(&mut self, key: &str, _conn: &mut dyn ConnectionLike);
}
pub struct SortedSetMigration {}

impl MigrationProcess for SortedSetMigration {
    fn migrate(&mut self, key: &str, redis_conn: &mut dyn ConnectionLike) {
        info!("KEY {}", key);
        let record_json: String = cmd("GET").arg(key).query(redis_conn).expect("Could not get the value.");
        let record: Record = serde_json::from_str(&record_json).expect("Could not deserialize.");

        info!(
            "Got key {} with time {} so timestamp is {}",
            record.id,
            record.date,
            record.date.and_hms(0, 0, 0).and_utc().timestamp()
        )
    }
}
