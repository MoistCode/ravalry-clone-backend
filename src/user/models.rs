use serde::{Deserialize, Serialize};

use crate::schema::users;

/// User properties for a user in the database. Should not be public facing.
#[derive(Debug, Serialize, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

/// User information meant to be public facing.
#[derive(Serialize)]
pub struct UserInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

/// User properties for a new user in the database. Should not be public facing.
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}