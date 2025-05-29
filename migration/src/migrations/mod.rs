use redis::ConnectionLike;

pub mod sorted_set;

pub trait MigrationProcess {
    fn migrate(&mut self, key: &str, _conn: &mut dyn ConnectionLike);
    fn get_start_version(&self) -> u64;
}
