use std::fmt::{Debug, Formatter};
use redis::ConnectionLike;

pub mod sorted_set;
pub mod rename_prefix;

pub trait MigrationProcess {
    fn migrate(&mut self, key: &str, _conn: &mut dyn ConnectionLike);
    fn get_start_version(&self) -> u64;
    fn get_description(&self) -> String;
    fn print_results(&mut self);
}

impl Debug for dyn MigrationProcess {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", &self.get_description())
    }
}