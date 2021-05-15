use crate::schema::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Exchange {
    pub id: i32,
    pub filename: String,
    pub usernamedst: String,
    pub usernamesrc: String,
    //pub fileid: i32,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Insertable, Debug)]
#[table_name = "exchanges"]
pub struct NewExchange<'a> {
    pub filename: &'a str,
    pub usernamedst: &'a str,
    pub usernamesrc: &'a str,
    //pub fileid: &'a i32,
    pub created_at: chrono::NaiveDateTime,
}
