use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use crate::schema::patterns;

#[derive(Debug, Serialize, Queryable, Insertable)]
pub struct Pattern {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewPattern {
    pub name: String,
}