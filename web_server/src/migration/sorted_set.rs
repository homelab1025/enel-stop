use log::info;
use redis::cmd;
use common::Record;

pub fn create_timestamp_sorted_set(key: &String, redis_conn: &mut redis::Connection) {
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