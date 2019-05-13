use rusqlite::*;
use crate::database_helper_utility as dhu;

#[derive(Debug)]
pub struct Version {
    pub table_name: String, //table_name is the table_name for this record.
    pub table_version: String, //table_version is the table version for this record.
}

pub fn _fields() -> String {
    String::from(
        ["table_name,table_version",
         ""].join("")
         )
} 
