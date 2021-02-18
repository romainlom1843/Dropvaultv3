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
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Token {
	pub token_id: i32,
    pub login: String,
    pub token: String,
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

#[derive(Insertable, Debug)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
	pub login: &'a str,
    pub token: &'a str,
    pub created_at: chrono::NaiveDateTime,
}



