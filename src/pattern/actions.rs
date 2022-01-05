use actix_web::{web};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::pattern::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find the pattern by uid and return the public
/// facing pattern information.
pub fn find_pattern_info_by_uid(
    uid: Uuid,
    conn: &SqliteConnection,
) -> Result<Option<models::Pattern>, DbError> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::patterns::dsl::*;

    let pattern = patterns
        .filter(id.eq(uid.to_string()))
        .first::<models::Pattern>(conn)
        .optional()?
        .unwrap();

    Ok(Some(pattern))
}

pub fn insert_new_pattern(
    form: &web::Json<models::NewPattern>,
    conn: &SqliteConnection
) -> Result<models::Pattern, DbError> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::patterns::dsl::*;

    let utc = Utc::now();
    let timestamp = utc.timestamp();

    let pattern = models::Pattern {
        id: Uuid::new_v4().to_string(),
        name: form.name.to_owned(),
        created_at: NaiveDateTime::from_timestamp(timestamp, 0),
    };

    diesel::insert_into(patterns).values(&pattern).execute(conn)?;

    Ok(pattern)
}