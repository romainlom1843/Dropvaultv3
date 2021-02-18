extern crate bcrypt;

use super::models::{NewUser, User, NewToken, Token};
use super::schema::users::dsl::*;
use super::schema::tokens::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use bcrypt::{DEFAULT_COST, hash/*, verify*/};
use crate::diesel::ExpressionMethods;


use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;





#[derive(Debug, Serialize, Deserialize)]
pub struct InputSignUp {
    pub username: String,
    pub email: String,
    pub passwd: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InputLogin {
    pub username: String,
    pub passwd: String,
}

//Recupère tous les utilisateurs
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().expect("pool database");
    let items = users.load::<User>(&conn)?;
    Ok(items)
}


//Sign up
pub async fn add_user( db: web::Data<Pool>,item: web::Json<InputSignUp>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
fn add_single_user(db: web::Data<Pool>,item: web::Json<InputSignUp>) -> Result<User, diesel::result::Error> {
    let conn = db.get().expect("db recup");
    let hashed = hash(&item.passwd, DEFAULT_COST).expect("password hashed");
    let new_user = NewUser {
       	username: &item.username,
        email: &item.email,
        hashe: &hashed,
        created_at: chrono::Local::now().naive_local(),
    };
    
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}



//Login
pub async fn get_user(db: web::Data<Pool>, item: web::Json<InputLogin>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, item))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
fn db_get_user_by_id(pool: web::Data<Pool>, item: web::Json<InputLogin>) -> Result<Token,diesel::result::Error> {
    let conn = pool.get().expect("db_recup");
    let hashed = hash(&item.passwd, DEFAULT_COST).expect("Password hashed");
    let user_id = users.select(id).filter(username.eq(&item.username)).filter(hashe.eq(&hashed)).get_result::<i32>(&conn);
   // if user_id == Error
    //{
     	let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").expect("Hmac created");
		let mut claims = BTreeMap::new();
		claims.insert("sub", &item.username);
		let token_str = claims.sign_with_key(&key).expect("token created");
		let new_token = NewToken {
		login : &item.username,
       	token: &token_str,
        created_at: chrono::Local::now().naive_local(),
    };
    
		
		 let res = insert_into(tokens).values(&new_token).get_result(&conn)?;
    	 Ok(res)
   // }
   // else{
    
    
    //}
    
   
}



//Delete
pub async fn delete_user(db: web::Data<Pool>,user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}








