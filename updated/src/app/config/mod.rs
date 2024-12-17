use std::env;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub struct Config {
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Config {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: &str, exp: usize) -> Self {
        Claims {
            sub: user_id.to_string(),
            exp,
        }
    }
}

pub fn encode_jwt(claims: &Claims, secret: &str) -> String {
    jsonwebtoken::encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to encode JWT")
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, String> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|err| err.to_string())
}
