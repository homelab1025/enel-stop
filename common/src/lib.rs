use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub date: NaiveDate,
    pub judet: String,
    pub localitate: String,
    pub title: String,
    pub description: String,
}
