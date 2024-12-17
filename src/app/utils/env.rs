use dotenv::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn get_required_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Environment variable '{}' not set!", key))
}
