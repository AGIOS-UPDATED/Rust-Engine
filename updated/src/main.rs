use actix_web::{web, App, HttpServer, HttpResponse, Responder};

// A simple handler that returns a greeting message
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))  // Route for the root URL
    })
    .bind("127.0.0.1:8080")?  // Bind to local IP and port 8080
    .run()
    .await
}
