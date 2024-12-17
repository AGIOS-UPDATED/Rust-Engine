use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::{config::{Claims, encode_jwt, Config}, models::user::{User, UserDB}};

#[derive(Deserialize)]
pub struct AuthRequest {
    email: String,
    password: String,
}

pub async fn sign_up(data: web::Json<AuthRequest>, db: web::Data<UserDB>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let user = User::new(&data.email, &data.password);

    match db.add_user(user) {
        Ok(_) => HttpResponse::Created().body("User created successfully"),
        Err(err) => HttpResponse::Conflict().body(err),
    }
}

pub async fn sign_in(data: web::Json<AuthRequest>, db: web::Data<UserDB>, config: web::Data<Config>) -> impl Responder {
    let db = db.lock().unwrap();
    if let Some(user) = db.find_by_email(&data.email) {
        if user.verify_password(&data.password) {
            let claims = Claims::new(&user.id, 3600);
            let token = encode_jwt(&claims, &config.jwt_secret);
            return HttpResponse::Ok().json(token);
        }
    }
    HttpResponse::Unauthorized().body("Invalid email or password")
}
