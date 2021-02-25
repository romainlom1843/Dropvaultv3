use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Insertable, Debug)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub filename: &'a str,
    pub username: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
