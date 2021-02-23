use crate::schema::*;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub passwd: String,
    pub created_at: chrono::NaiveDateTime,
}



#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hashe: &'a String,
    pub created_at: chrono::NaiveDateTime,
}




