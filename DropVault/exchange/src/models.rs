use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Pubkey {
    pub id: i32,
    pub pubkey: String,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Insertable, Debug)]
#[table_name = "pubkeys"]
pub struct NewPubKey<'a> {
    pub pubkey: &'a str,
    pub username: &'a str,
    pub created_at: chrono::NaiveDateTime,
}
/*#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Exchange {
    pub id: i32,
    pub filename: String,
    pub username: String,
    pub fileid: i32,
    pub login : String,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Insertable, Debug)]
#[table_name = "exchanges"]
pub struct NewExchange<'a> {
    pub filename: &'a str,
    pub username: &'a str,
    pub fileid: &'a i32,
    pub login: &'a str,
    pub created_at: chrono::NaiveDateTime,
}*/
