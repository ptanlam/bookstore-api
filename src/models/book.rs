use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}
