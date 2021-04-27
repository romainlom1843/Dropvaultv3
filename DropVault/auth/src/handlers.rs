extern crate bcrypt;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;

use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use bcrypt::{ hash, verify};
use crate::diesel::ExpressionMethods;
use diesel::NotFound;

use chrono::prelude::*;
use jwt::{encode, Header, Algorithm};
use rand::{distributions::Alphanumeric, Rng};


use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2
};
use rand_core::OsRng;

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}



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
#[derive(Debug, Serialize, Deserialize)]
pub struct PassLog {
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
    let hashed = hash(&item.passwd, 6).expect("password hashed");
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
fn db_get_user_by_id(pool: web::Data<Pool>, item: web::Json<InputLogin>) -> Result<String,diesel::result::Error> {
    let conn = pool.get().expect("db_recup");
    let hashed = users.select(hashe).filter(username.eq(&item.username)).get_result::<String>(&conn).expect("Hashed");
    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(300)).expect("valid timestamp").timestamp();
    let valid = verify(&item.passwd, &hashed).expect("Password verify");
    if valid == true
    {
     	let my_claims = Claims {
		sub: item.username.to_owned(),
		company: "DropVault".to_owned(),
		exp: expiration as usize,
    	};

     	let mut header = Header::default();
     	let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
		header.kid = Some(s.to_owned());
		header.alg = Algorithm::HS256;
		let token = encode(header, &my_claims, "mon_secret".as_ref()).expect("token created");

    	Ok(token)
    }
else{
    return Err(NotFound);
    // Error 403
    }  
}

// Key derivation 

pub async fn derive_passwd(item: web::Json<PassLog>) -> HttpResponse {


  	let password = item.passwd.as_bytes();
	let salt = SaltString::generate(&mut OsRng);
	
	// Hash password to PHC string ($pbkdf2-sha256$...)
	let password_hash = Pbkdf2.hash_password_simple(password, salt.as_ref()).expect("derivation").to_string();
	//println!("{}", password_hash);
	
	// Verify password against PHC string
	let parsed_hash = PasswordHash::new(&password_hash).expect("hash verified");
	//println!("{}", parsed_hash);
	HttpResponse::Ok().json(password_hash)
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








