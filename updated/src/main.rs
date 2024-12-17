use actix_web::{App, HttpServer, web};
use std::sync::Mutex;
use crate::{config::Config, models::user::UserDB, routes::configure_routes};

mod config;
mod controllers;
mod middlewares;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let user_db = web::Data::new(Mutex::new(UserDB::new()));
    let config = web::Data::new(Config::from_env());

    HttpServer::new(move || {
        App::new()
            .app_data(user_db.clone())
            .app_data(config.clone())
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
