#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware};
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

pub mod pattern;
pub mod schema;
pub mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = establish_connection_manager();

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");
    
    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start the HTTP server.
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            // Favorite routes
            // Pattern routes
            .service(pattern::routes::get_pattern)
            .service(pattern::routes::add_pattern)
            // User routes
            .service(user::routes::get_user)
            .service(user::routes::add_user)
    })
    .bind(&bind)?
    .run()
    .await
}

pub fn establish_connection_manager() -> ConnectionManager<SqliteConnection> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");

    ConnectionManager::<SqliteConnection>::new(&database_url)
}