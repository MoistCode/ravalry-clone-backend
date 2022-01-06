use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use crate::favorite::models;
use crate::pattern;
use crate::user;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Find all favorites by pattern uid.
pub fn find_favorites_by_pattern_uid(
    uid: Uuid,
    conn: &SqliteConnection
) -> Result<Option<Vec<models::Favorite>>, DbError> {
    use crate::schema::favorites::dsl::*;

    let list_of_favorites = favorites
        .filter(pattern_id.eq(uid.to_string()))
        .load::<models::Favorite>(conn)
        .optional()?
        .unwrap();

    Ok(Some(list_of_favorites))
}

/// Find all favorites by user uid.
pub fn find_favorites_by_user_uid(
    uid: Uuid,
    conn: &SqliteConnection
) -> Result<Option<Vec<models::Favorite>>, DbError> {
    use crate::schema::favorites::dsl::*;

    let list_of_favorites = favorites
        .filter(user_id.eq(uid.to_string()))
        .load::<models::Favorite>(conn)
        .optional()?
        .unwrap();

    Ok(Some(list_of_favorites))
}

pub fn insert_new_favorite(
    form: &web::Json<models::NewFavorite>,
    conn: &SqliteConnection,
) -> Result<Option<models::Favorite>, DbError> {
    use crate::schema::favorites::dsl::*;

    // TODO: Return an error stating that the user or pattern does not exist.
    let user_uid = Uuid::parse_str(&form.user_id)?;
    let pattern_uid = Uuid::parse_str(&form.pattern_id)?;
    user::actions::find_user_info_by_uid(user_uid, &conn)?;
    pattern::actions::find_pattern_info_by_uid(pattern_uid, &conn)?;

    let favorite = models::Favorite {
        id: Uuid::new_v4().to_string(),
        pattern_id: form.pattern_id.to_owned(),
        user_id: form.user_id.to_owned(),
    };

    diesel::insert_into(favorites).values(&favorite).execute(conn)?;

    Ok(Some(favorite))
}