use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub address: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            db_url: env::var("DB_URL").expect("DB_URL must be set"),
            address: env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        }
    }
}
