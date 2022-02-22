
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::patterns;
use crate::user::models::User;

#[derive(Associations, Debug, Identifiable, Serialize, Queryable, Insertable)]
#[belongs_to(User)]
pub struct Pattern {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub homepage_url: String,
    pub highlight_image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub num_favorites: i32,
    pub times_visited_in_24_hours: i32,
}

#[derive(Debug, Serialize)]
pub struct PatternWithUserInfo {
    pub user_first_name: String,
    pub user_last_name: String,
    pub name: String,
    pub homepage_url: String,
    pub highlight_image_url: Option<String>,
    pub num_favorites: i32,
    pub times_visited_in_24_hours: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewPattern {
    pub name: String,
    pub user_id: String,
}