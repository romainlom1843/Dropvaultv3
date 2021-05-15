#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};



 
mod echange;
mod schema;
mod models;
mod auth;
mod errors;

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
       	.route("/exchange", web::post().to(echange::exchange))        	        	
        	.route("/delete/{id}", web::delete().to(echange::delete_exchange))
        	.route("/exchange/{username}", web::get().to(echange::get_exchange))
        	.route("/key", web::post().to(echange::upl_key))
        	.route("/recup/{filename}", web::get().to(echange::dwl_key))
        	.route("/user/{filename}", web::get().to(echange::get_user))
        	.route("/id/{user_name}/{file_name}", web::get().to(echange::get_exchange_id))
        	.route("/remove/{id}", web::delete().to(echange::remove_key))
         })
        
    .bind("0.0.0.0:9001")?
    .run()
    .await
}



