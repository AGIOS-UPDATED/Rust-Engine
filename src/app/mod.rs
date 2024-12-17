use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use std::env;
use colored::*;  
pub fn run() {
    dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());
    println!("{}", format!("Starting server at http://{} 🚀", server_address).green().bold());
    println!("{}",format!(" Lighting fast Engine 🦀").purple().bold());
    actix_web::rt::System::new().block_on(async {
        HttpServer::new(|| {
            App::new()
                // Enable logger middleware
                .wrap(Logger::default())
                // Define the main route (GET /)
                .route("/", web::get().to(main_handler))
        })
        .bind(&server_address)
        .expect("Failed to bind address")
        .run()
        .await
        .unwrap();
    });
}
async fn main_handler() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the main route Updated ")
}
