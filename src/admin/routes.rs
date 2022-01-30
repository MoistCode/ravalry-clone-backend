use actix_web::{Error, HttpResponse, get, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::admin;

#[get("/admin/populate")]
pub async fn populate(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        admin::actions::populate_database(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}