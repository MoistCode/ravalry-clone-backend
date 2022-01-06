use serde::{Deserialize, Serialize};

use crate::schema::favorites;

#[derive(Debug, Serialize, Queryable, Insertable)]
pub struct Favorite {
    pub id: String,
    pub pattern_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewFavorite {
    pub pattern_id: String,
    pub user_id: String,
}