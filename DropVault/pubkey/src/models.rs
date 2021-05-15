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

