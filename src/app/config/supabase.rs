use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
pub struct SupabaseConfig {
    pub url: String,
    pub api_key: String,
}

impl SupabaseConfig {
    pub fn new() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::from_env()
    }
}
