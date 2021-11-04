use actix_web::{web};
use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find the user by uid and return that user.
pub fn find_user_by_uid (
    uid: Uuid,
    conn: &SqliteConnection,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row for a new user and
/// return the result.
pub fn insert_new_user(
    // Prevent collision with `name` column imported inside the function.
    form: &web::Json<models::NewUser>,
    conn: &SqliteConnection,
) -> Result<models::User, DbError> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let hash_pw = hash(form.password.to_owned(), DEFAULT_COST)?;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        first_name: form.first_name.to_owned(),
        last_name: form.last_name.to_owned(),
        username: form.username.to_owned(),
        password: hash_pw,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}