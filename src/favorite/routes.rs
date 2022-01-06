use actix_web::{Error, HttpResponse, post, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::favorite;

#[post("/favorite")]
pub async fn add_favorite(
    pool: web::Data<DbPool>,
    form: web::Json<favorite::models::NewFavorite>,
) -> Result<HttpResponse, Error> {
    let favorite = web::block(move || {
        let conn = pool.get()?;
        favorite::action::insert_new_favorite(&form, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(favorite))
}