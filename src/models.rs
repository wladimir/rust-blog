use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}
