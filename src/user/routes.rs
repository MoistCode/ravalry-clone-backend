use actix_web::{Error, get, HttpResponse, post, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::favorite;
use crate::user;

/// Find user by their UID.
#[get("/user/{user_id}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();

    // Use web::block to offload blocking Diesel code without blocking the
    // server thread.
    let user = web::block(move || {
        let conn = pool.get()?;
        user::actions::find_user_info_by_uid(user_uid, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// Inserts a new user with the name defined in the form.
#[post("/user")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<user::models::NewUser>,
) -> Result<HttpResponse, Error> {
    // Use web::block to offload blocking Diesel code without blocking the
    // server thread.
    let user = web::block(move || {
        let conn = pool.get()?;
        user::actions::insert_new_user(&form, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(user))
}

/// Gets all user favorites.
#[get("/user/{user_id}/favorites")]
pub async fn get_user_favorites(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let favorites = web::block(move || {
        let conn = pool.get()?;
        favorite::actions::find_favorites_by_user_uid(user_uid.into_inner(), &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(favorites))
}