#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // Set up database connection pool.
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create databasse connection pool.");
    
    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start the HTTP server.
    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            // Favorite routes
            // User routes
            .service(user::routes::get_user)
            .service(user::routes::add_user)
    })
    .bind(&bind)?
    .run()
    .await
}