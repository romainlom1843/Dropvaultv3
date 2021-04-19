use super::models::{NewFile, File};
use super::schema::files::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::diesel::ExpressionMethods;
use diesel::NotFound;
use std::io::Write;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFile {
    pub filename: String,
    pub username: String,
    pub sizing: String,
    pub ext: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FormData{
	pub content : String,	
	pub key : String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo{
	pub user_name : String,
	pub file_name : String,
	
}


//Recuperer id d'un file
pub async fn get_file_id(db: web::Data<Pool>, info: web::Path<FileInfo>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_files_id(db, info.user_name.to_string() , info.file_name.to_string()))
        	.await
        	.map(|file| HttpResponse::Ok().json(file))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_files_id(pool: web::Data<Pool>, user_name: String, file_name: String) -> Result<i32, diesel::result::Error> {
	
    
    let conn = pool.get().expect("database pool");
    let file_id = files.select(id).filter(username.eq(&user_name)).filter(filename.eq(&file_name)).first::<i32>(&conn)?;
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
        username: &item.username,
        sizing: &item.sizing,
    	ext: &item.ext,
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
        username: &item.username,
        sizing: &item.sizing,
    	ext: &item.ext,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(files).values(&new_file).get_result(&conn)?;
    Ok(res)
    }
   
}
pub async fn upl_content(id_c: web::Path<u8>, item: web::Json<FormData>)-> impl Responder{

	
	let path = "../stock/";
	let path2= "../key/";
	let file_name= path.to_owned() + &id_c.to_string();
	let file_name2= path2.to_owned() + &id_c.to_string();
	//let mut file1 = std::fs::File::open(&file_name);
	
	//if file1.is_ok(){
		//let mut file2 = std::fs::OpenOptions::new().write(true).append(true).open(&file_name).expect("file already created");
		//file2.write_all(&item.content.as_bytes()).expect("Don't write");
		
	//}
	//else{
	let mut file = std::fs::File::create(&file_name).expect("file created");
	file.write_all(&item.content.as_bytes()).expect("Don't write");
	let mut file2 = std::fs::File::create(&file_name2).expect("file created");
	file2.write_all(&item.key.as_bytes()).expect("Don't write");
		
	//}
	HttpResponse::Ok().body("Response")
}


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
pub async fn dwl_content(file_id: web::Path<i32>)-> impl Responder{

	let path = "../stock/";
	let file_name= path.to_owned() + &file_id.to_string();
	let mut file = std::fs::File::open(&file_name).expect("file don't load");
	/*let metadata= std::fs::metadata(&file_name).expect("unable to read metadata");
	let mut buffer= vec![0; metadata.len() as usize];*/
	let mut contents =String::new();
	file.read_to_string(&mut contents).expect("File read");
	HttpResponse::Ok().body(contents)
}



// Récupération des données
//Recuperer tous les noms par username
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
//Recuperer toutes les tailles par username
pub async fn get_size(db: web::Data<Pool>, file_username: web::Path<String>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_size_by_user(db, file_username.to_string()))
        	.await
        	.map(|file| HttpResponse::Ok().json(file))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_size_by_user(pool: web::Data<Pool>, file_username: String) -> Result<Vec<String>, diesel::result::Error> {
	
    let conn = pool.get().expect("database pool");
    let items = files.select(sizing).filter(username.eq(&file_username)).load::<String>(&conn)?;
    Ok(items)
    
}
//Recuperer toutes les types par username
pub async fn get_type(db: web::Data<Pool>, file_username: web::Path<String>) -> Result<HttpResponse, Error> {

	Ok(
		web::block(move || get_type_by_user(db, file_username.to_string()))
        	.await
        	.map(|file| HttpResponse::Ok().json(file))
        	.map_err(|_| HttpResponse::InternalServerError())?)
   
}
fn get_type_by_user(pool: web::Data<Pool>, file_username: String) -> Result<Vec<String>, diesel::result::Error> {
	
    let conn = pool.get().expect("database pool");
    let items = files.select(ext).filter(username.eq(&file_username)).load::<String>(&conn)?;
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
pub async fn remove_content(file_id: web::Path<i32>)-> impl Responder{

	let path = "../stock/";
	let file_name= path.to_owned() + &file_id.to_string();
	std::fs::remove_file(&file_name).expect("file don't load");
	HttpResponse::Ok().body("Delete")
	
}




// Echange file
pub async fn echange(  db: web::Data<Pool>, item: web::Json<InputFile>) -> Result<HttpResponse, Error>{
	
    Ok(web::block(move || echange_file(db, item))
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
        username: &item.username,
        sizing: &item.sizing,
    	ext: &item.ext,
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

