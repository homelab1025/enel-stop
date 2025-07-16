use redis::ConnectionLike;
use std::fmt::{Debug, Formatter};
use common::configuration::ServiceConfiguration;

pub mod postgresql;
pub mod recreate_sorted_set;
pub mod rename_prefix;
pub mod sorted_set;
pub trait MigrationProcess {
    /// Run migration action before looping thru the key.
    ///
    /// # Arguments
    ///
    /// * `conn`: Redis connection.
    ///
    /// returns: ()
    ///
    fn migrate(&mut self, _conn: &mut dyn ConnectionLike, _service_config: &ServiceConfiguration) {}
    fn migrate_key(&mut self, key: &str, conn: &mut dyn ConnectionLike);
    fn get_start_version(&self) -> u64;
    fn get_description(&self) -> String;
    fn print_results(&self);
}

impl Debug for dyn MigrationProcess {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", &self.get_description())
    }
}
