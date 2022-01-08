#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware};
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};

pub mod favorite;
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
            .service(favorite::routes::add_favorite)
            // Pattern routes
            .service(pattern::routes::get_pattern)
            .service(pattern::routes::add_pattern)
            .service(pattern::routes::get_pattern_favorited_users)
            // User routes
            .service(user::routes::get_user)
            .service(user::routes::add_user)
            .service(user::routes::get_user_favorites)
    })
    .bind(&bind)?
    .run()
    .await
}

fn establish_connection_manager() -> ConnectionManager<SqliteConnection> {
    let database_url = get_database_url();
    ConnectionManager::<SqliteConnection>::new(&database_url)
}

#[cfg(debug_assertions)]
fn get_database_url() -> std::string::String {
    println!("Getting dev database URL...");
    dotenv::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.")
}

#[cfg(not(debug_assertions))]
fn get_database_url() -> std::string::String {
    println!("Getting production database URL...");
    "production.db".to_string()
}