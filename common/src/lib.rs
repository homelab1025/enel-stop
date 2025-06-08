pub mod configuration;
pub mod persistence;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Deserialize)]
pub struct Record {
    pub id: String,
    pub date: NaiveDate,
    pub judet: String,
    pub localitate: String,
    pub title: String,
    pub description: String,
}
