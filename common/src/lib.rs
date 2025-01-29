use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub date: String,
    pub judet: String,
    pub localitate: String,
    pub title: String,
    pub description: String,
}
