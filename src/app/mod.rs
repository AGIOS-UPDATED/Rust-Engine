use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use std::env;
use colored::*;  // Add this line to bring colored into scope

// mod controllers;  // Assuming you have controllers for your app
// mod models;       // Assuming you have models for interacting with your database

// This is your main function that starts the server
pub fn run() {
    // Load environment variables
    dotenv().ok();

    // Get the server address from environment variables or use a default
    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());

    // Start the Actix Web server
    println!("{}", format!("Starting server at http://{} 🚀", server_address).green().bold());


    // Start the HTTP server with routes, middleware, and controller logic
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

// Define a handler for the main route
async fn main_handler() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the main route Updated ")
}
