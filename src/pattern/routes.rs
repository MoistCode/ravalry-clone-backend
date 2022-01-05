use actix_web::{Error, get, HttpResponse, post, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::pattern;

#[get("/pattern/{pattern_id}")]
pub async fn get_pattern (
    pool: web::Data<DbPool>,
    pattern_uid: web::Path<Uuid>
) -> Result<HttpResponse, Error> {
    let pattern_uid = pattern_uid.into_inner();

    // Use web::block to offload blocking Diesel code without blocking the
    // server thread.
    let pattern = web::block(move || {
        let conn = pool.get()?;
        pattern::actions::find_pattern_info_by_uid(pattern_uid, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(pattern) = pattern {
        Ok(HttpResponse::Ok().json(pattern))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No pattern found with uid: {}", pattern_uid));
        Ok(res)
    }
}

#[post("/pattern")]
pub async fn add_pattern(
    pool: web::Data<DbPool>,
    form: web::Json<pattern::models::NewPattern>,
) -> Result<HttpResponse, Error> {
    let pattern = web::block(move || {
        let conn = pool.get()?;
        pattern::actions::insert_new_pattern(&form, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(pattern))
}