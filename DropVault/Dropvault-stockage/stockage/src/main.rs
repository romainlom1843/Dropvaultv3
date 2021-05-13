
#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};



 
mod files;
mod errors;
mod schema;
mod models;
mod auth;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;


async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
               Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
        
    HttpServer::new(move || {
     
  let auth = HttpAuthentication::bearer(validator);
 
     
        App::new()
       	.wrap(auth)
        	.data(pool.clone())
        	.route("/file/{user_name}/{file_name}", web::get().to(files::get_file_id))
        	.route("/ext/{file_name}", web::get().to(files::get_type))
            	.route("/files/{username}", web::get().to(files::get_files))
            	.route("/size/{username}", web::get().to(files::get_size))
            	.route("/type/{username}", web::get().to(files::get_type))
            	.route("/download/{id}", web::get().to(files::download))
            	.route("/files", web::post().to(files::upload))
            	.route("/upload/{id}", web::post().to(files::upl_content))
            	.route("/dwl/{id}", web::get().to(files::dwl_content))
            	.route("/key/{id}", web::get().to(files::dwl_key))
            	.route("/files/{id}", web::delete().to(files::delete_file))
            	.route("/remove/{id}", web::delete().to(files::remove_content))
    })
    .bind("0.0.0.0:8084")?
    .run()
    .await
}



