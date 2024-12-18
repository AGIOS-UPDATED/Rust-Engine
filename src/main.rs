
mod app;

#[actix_web::main] // This attribute makes the main function async and sets up Actix runtime.
async fn main() -> std::io::Result<()> {
    // Just call app::run() here.
    app::run().await
}
