use super::models::{NewPubKey, Pubkey};
use super::schema::pubkeys::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use crate::diesel::ExpressionMethods;


#[derive(Debug, Serialize, Deserialize)]
pub struct InputFile {
    pub pubkey: String,
    pub username: String,
  
}

// Create PubKey
pub async fn create_pubkey( db: web::Data<Pool>, item: web::Json<InputFile>,) -> Result<HttpResponse, Error>{
	
    Ok(web::block(move || upload_pubkey(db, item))
        .await
        .map(|Pubkey| HttpResponse::Created().json(Pubkey))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
fn upload_pubkey(db: web::Data<Pool>, item: web::Json<InputFile>) -> Result<Pubkey, diesel::result::Error> {
    let conn = db.get().expect("pool database");
    let id_pubkey = pubkeys.select(id).filter(username.eq(&item.username)).get_result::<i32>(&conn);
   
   
    
if id_pubkey.is_ok() == false
  {
    let new_pubkey = NewPubKey {
        pubkey: &item.pubkey,
        username: &item.username,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(pubkeys).values(&new_pubkey)/*.on_conflict((filename, username)).do_update().set(content.eq(&item.content))*/.get_result::<Pubkey>(&conn)?;
    Ok(res)
    }
  else
    {
     //let update = diesel::update(files.find(id_file)).set(content.eq(&item.content)).get_result(&conn)?;
     //Ok(update)
       delete_pubkey_id( db,  id_pubkey?)?;
       let new_pubkey = NewPubKey {
        pubkey: &item.pubkey,
        username: &item.username,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(pubkeys).values(&new_pubkey).get_result(&conn)?;
    Ok(res)
    }
   
}

// Delete une pubkey par id
pub async fn delete_pubkey( db: web::Data<Pool>,  pubkey_id: web::Path<i32>) ->  Result<HttpResponse, Error> {
      Ok(
        web::block(move || delete_pubkey_id(db, pubkey_id.into_inner()))
            .await
            .map(|Pubkey| HttpResponse::Ok().json(Pubkey))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
fn delete_pubkey_id(db: web::Data<Pool>, pubkey_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(pubkeys.find(pubkey_id)).execute(&conn)?;
    Ok(count)
}

//get public key by username
pub async fn get_pubkey(db: web::Data<Pool>, pubkey_username: web::Path<String>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_pubkey_by_user(db, pubkey_username.to_string()))
        	.await
        	.map(|Pubkey| HttpResponse::Ok().json(Pubkey))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_pubkey_by_user(pool: web::Data<Pool>, pubkey_username: String) -> Result<String, diesel::result::Error> {
	
    let conn = pool.get().expect("database pool");
    let items = pubkeys.select(pubkey).filter(username.eq(&pubkey_username)).first::<String>(&conn)?;
    Ok(items)
    
}
