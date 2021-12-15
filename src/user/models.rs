use serde::{Deserialize, Serialize};

use crate::user::schema::users;

#[derive(Debug, Serialize, Queryable, Insertable)]
/// User properties for a user in the database. Should not be public facing.
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
/// User information meant to be public facing.
pub struct UserInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
/// User properties for a new user in the database. Should not be public facing.
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}