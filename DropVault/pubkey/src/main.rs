#[macro_use]
extern crate diesel;

use actix_web::{ web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};



 
mod exchange;
mod schema;
mod models;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;





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
     
 
     
        App::new()
       	
       	
       	.route("/pubkey", web::post().to(exchange::create_pubkey))
        	.data(pool.clone())        	
        	.route("/delete/{id}", web::delete().to(exchange::delete_pubkey))
        	.route("/pubkey/{username}", web::get().to(exchange::get_pubkey))
        	
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}



