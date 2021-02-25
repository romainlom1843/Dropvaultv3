use super::models::{NewFile, File};
use super::schema::files::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::diesel::ExpressionMethods;
use diesel::NotFound;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFile {
    pub filename: String,
    pub content: String,
    pub username: String,
}


//Recuperer id d'un file
pub async fn get_file_id(db: web::Data<Pool>, file_name: web::Path<String>/*, user_name: web::Path<String>*/) -> Result<HttpResponse, Error> {
 println!("ici2");
	Ok(
		web::block(move || get_files_id(db, file_name.to_string()/*, user_name.to_string()*/))
        	.await
        	.map(|file| HttpResponse::Ok().json(file))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_files_id(pool: web::Data<Pool>, file_name: String/*, user_name: String*/) -> Result<i32, diesel::result::Error> {
	
    println!("ici");
    let conn = pool.get().expect("database pool");
    let file_id = files.select(id).filter(filename.eq(&file_name))/*.filter(username.eq(&user_name))*/.first::<i32>(&conn)?;
    Ok(file_id)
    
}





// Upload file
pub async fn upload(  db: web::Data<Pool>, item: web::Json<InputFile>,) -> Result<HttpResponse, Error>{
	
    Ok(web::block(move || upload_file(db, item))
        .await
        .map(|file| HttpResponse::Created().json(file))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
fn upload_file(db: web::Data<Pool>, item: web::Json<InputFile>) -> Result<File, diesel::result::Error> {
    let conn = db.get().expect("pool database");
    let id_file = files.select(id).filter(filename.eq(&item.filename)).filter(username.eq(&item.username)).get_result::<i32>(&conn);
   
   
    
if id_file.is_ok() == false
  {
    let new_file = NewFile {
        filename: &item.filename,
        content: &item.content,
        username: &item.username,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(files).values(&new_file)/*.on_conflict((filename, username)).do_update().set(content.eq(&item.content))*/.get_result::<File>(&conn)?;
    Ok(res)
    }
  else
    {
     //let update = diesel::update(files.find(id_file)).set(content.eq(&item.content)).get_result(&conn)?;
     //Ok(update)
       delete_file_id( db,  id_file?)?;
       let new_file = NewFile {
        filename: &item.filename,
        content: &item.content,
        username: &item.username,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(files).values(&new_file).get_result(&conn)?;
    Ok(res)
    }
   
}
/*pub async fn uplContent(content: String, id : i32)-> Result<HttpResponse, Error>{

let file_name = "../../stock" + id.to_string();
let mut file = File::create(&file_name).expect("file created");
file.write_all(content).encrypt("Don't write");
}*/




// Download = get un fichier par id
pub async fn download( db: web::Data<Pool>, file_id: web::Path<i32>) ->  Result<HttpResponse, Error>  {	
   
    
    Ok( 
        web::block(move || db_download(db, file_id.into_inner()))
            .await
            .map(|file| HttpResponse::Ok().json(file))
            .map_err(|_| HttpResponse::InternalServerError())?)
            
           
    
}
fn db_download(pool: web::Data<Pool>, file_id: i32) -> Result<File, diesel::result::Error> {
 
    let conn = pool.get().unwrap();
    files.find(file_id).get_result::<File>(&conn)
   
    
}



//Recuperer tous les fichiers par username
pub async fn get_files(db: web::Data<Pool>, file_username: web::Path<String>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_files_by_user(db, file_username.to_string()))
        	.await
        	.map(|file| HttpResponse::Ok().json(file))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_files_by_user(pool: web::Data<Pool>, file_username: String) -> Result<Vec<String>, diesel::result::Error> {
	
    let conn = pool.get().expect("database pool");
    let items = files.select(filename).filter(username.eq(&file_username)).load::<String>(&conn)?;
    Ok(items)
    
}




// Delete un fichier par id
pub async fn delete_file( db: web::Data<Pool>,  file_id: web::Path<i32>) ->  Result<HttpResponse, Error> {
      Ok(
        web::block(move || delete_file_id(db, file_id.into_inner()))
            .await
            .map(|file| HttpResponse::Ok().json(file))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
fn delete_file_id(db: web::Data<Pool>, file_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = delete(files.find(file_id)).execute(&conn)?;
    Ok(count)
}






// Echange file
pub async fn echange(  db: web::Data<Pool>, item: web::Json<InputFile>) -> Result<HttpResponse, Error>{
	
    Ok(web::block(move || upload_file(db, item))
        .await
        .map(|file| HttpResponse::Created().json(file))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
fn echange_file(db: web::Data<Pool>, item: web::Json<InputFile>) -> Result<File, diesel::result::Error> {
    let conn = db.get().expect("pool database");
    let id_file = files.select(id).filter(filename.eq(&item.filename)).filter(username.eq(&item.username)).get_result::<i32>(&conn);
   
   
    
if id_file.is_ok() == false
  {
    let new_file = NewFile {
        filename: &item.filename,
        content: &item.content,
        username: &item.username,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(files).values(&new_file)/*.on_conflict((filename, username)).do_update().set(content.eq(&item.content))*/.get_result::<File>(&conn)?;
    Ok(res)
    }
  else
    {
     return Err(NotFound);
    }
   
}

