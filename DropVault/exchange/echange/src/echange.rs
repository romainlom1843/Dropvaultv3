use super::models::{NewExchange, Exchange};
use super::schema::exchanges::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::diesel::ExpressionMethods;
use std::io::Write;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo{
	pub user_name : String,
	pub file_name : String,
	
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InputFile {
    pub filename: String,
    pub usernamedst: String,
    pub usernamesrc: String,
   
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FormData{
	pub filename : String,	
	pub key : String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Input{
	pub filename : String,	
	
}

// Create PubKey
pub async fn exchange( db: web::Data<Pool>, item: web::Json<InputFile>,) -> Result<HttpResponse, Error>{
	
    Ok(web::block(move || upload_exchange(db, item))
        .await
        .map(|Exchange| HttpResponse::Created().json(Exchange))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
fn upload_exchange(db: web::Data<Pool>, item: web::Json<InputFile>) -> Result<Exchange, diesel::result::Error> {
    let conn = db.get().expect("pool database");
    let id_exchange = 		exchanges.select(id).filter(usernamesrc.eq(&item.usernamesrc)).filter(filename.eq(&item.filename)).get_result::<i32>(&conn);
   
   
    
if id_exchange.is_ok() == false
  {
    let new_exchange = NewExchange {
        filename: &item.filename,
        usernamedst: &item.usernamedst,
        usernamesrc: &item.usernamesrc,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(exchanges).values(&new_exchange)/*.on_conflict((filename, username)).do_update().set(content.eq(&item.content))*/.get_result::<Exchange>(&conn)?;
    Ok(res)
    }
  else
    {
     //let update = diesel::update(files.find(id_file)).set(content.eq(&item.content)).get_result(&conn)?;
     //Ok(update)
       delete_exchange_id( db,  id_exchange?)?;
       let new_exchange = NewExchange {
        filename: &item.filename,
        usernamedst: &item.usernamedst,
        usernamesrc: &item.usernamesrc,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(exchanges).values(&new_exchange).get_result::<Exchange>(&conn)?;
    Ok(res)
    }
   
}

// Delete un echange par id
pub async fn delete_exchange( db: web::Data<Pool>,  exchange_id: web::Path<i32>) ->  Result<HttpResponse, Error> {
      Ok(
        web::block(move || delete_exchange_id(db, exchange_id.into_inner()))
            .await
            .map(|Exchange| HttpResponse::Ok().json(Exchange))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
fn delete_exchange_id(db: web::Data<Pool>, exchange_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(exchanges.find(exchange_id)).execute(&conn)?;
    Ok(count)
}

//get echange by username
pub async fn get_exchange(db: web::Data<Pool>, exchange_username: web::Path<String>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_exchange_by_user(db, exchange_username.to_string()))
        	.await
        	.map(|Exchange| HttpResponse::Ok().json(Exchange))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_exchange_by_user(pool: web::Data<Pool>, exchange_username: String) -> Result<Vec<String>, diesel::result::Error> {
	
    let conn = pool.get().expect("database pool");
    let items = exchanges.select(filename).filter(usernamedst.eq(&exchange_username)).load::<String>(&conn)?;
    Ok(items)
    
}

pub async fn upl_key( item: web::Json<FormData>)-> impl Responder{

	
	
	let path= "../key/";
	let file_name= path.to_owned() + &item.filename.to_owned();	

	let mut file = std::fs::File::create(&file_name).expect("file created");
	file.write_all(&item.key.as_bytes()).expect("Don't write");
		
	
	HttpResponse::Ok().body("Response")
}
pub async fn dwl_key(file_name: web::Path<String>) -> impl Responder{
	
	
	let path2 = "../key/";
	let file_name2=path2.to_owned() + &file_name.to_string();
	let mut file2 = std::fs::File::open(&file_name2).expect("file don't load");
	let mut contents2= String::new();
	file2.read_to_string(&mut contents2).expect("File read");
	HttpResponse::Ok().body(contents2)
}

//Recuperer user source par filename
pub async fn get_user(db: web::Data<Pool>, file_name: web::Path<String>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_user_by_filename(db, file_name.to_string()))
        	.await
        	.map(|Exchange| HttpResponse::Ok().json(Exchange))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_user_by_filename(pool: web::Data<Pool>, file_name: String) -> Result<String, diesel::result::Error> {
	
    let conn = pool.get().expect("database pool");
    let items = exchanges.select(usernamesrc).filter(filename.eq(&file_name)).first::<String>(&conn)?;
    Ok(items)
    
}

//Recuperer id d'un file
pub async fn get_exchange_id(db: web::Data<Pool>, info: web::Path<FileInfo>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_exchanges_id(db, info.user_name.to_string() , info.file_name.to_string()))
        	.await
        	.map(|Exchange| HttpResponse::Ok().json(Exchange))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_exchanges_id(pool: web::Data<Pool>, user_name: String, file_name: String) -> Result<i32, diesel::result::Error> {
	
    
    let conn = pool.get().expect("database pool");
    let exchange_id = exchanges.select(id).filter(usernamedst.eq(&user_name)).filter(filename.eq(&file_name)).first::<i32>(&conn)?;
    Ok(exchange_id)
    
}
pub async fn remove_key(file_name: web::Path<String>)-> impl Responder{


		
	let path2 = "../key/";
	let file_name2=path2.to_owned() + &file_name.to_string();
	std::fs::remove_file(&file_name2).expect("file don't load");


	HttpResponse::Ok().body("Delete")
	
}
