mod config; // Keep this line, as you need config internally
pub mod controllers;
pub mod routes;
pub mod api_doc;

use actix_web::{App, HttpServer, middleware::Logger, web, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;
use colored::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::sync::Arc;
use postgrest::Postgrest;

struct AppState {
    supabase_client: Arc<Postgrest>
}

async fn init() -> Result<Arc<Postgrest>, Box<dyn std::error::Error>> {
    dotenv().ok();

    let supabase_url = env::var("SUPABASE_URL")?;
    let supabase_key = env::var("SUPABASE_KEY")?;

    let supabase_client = config::supabase::init_supabase(&supabase_url, &supabase_key);
    println!("{}", "Database connection established successfully! 🎉".blue().bold());
    Ok(supabase_client)
}

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());

    let supabase_client = match init().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("{}", format!("Initialization error: {}", e).red().bold());
            std::process::exit(1);
        }
    };

    println!("{}", format!("Starting server at http://{} 🚀", server_address).green().bold());
    println!("{}", format!("Lighting fast Engine 🦀").purple().bold());

    let openapi = api_doc::ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                supabase_client: supabase_client.clone()
            }))
            .route("/", web::get().to(main_handler))
            .configure(routes::auth_route::init)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone())
            )
    })
    .bind(&server_address)?
    .run()
    .await
}

async fn main_handler() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the main route!")
}
