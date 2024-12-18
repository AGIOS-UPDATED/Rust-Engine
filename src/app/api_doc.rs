use utoipa::OpenApi;
use crate::app::controllers::auth_controller;
use crate::app::controllers::rate_controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth controller paths
        auth_controller::login_handler,
        auth_controller::logout_handler,
        auth_controller::register_handler,
        
        // Rate controller paths
        rate_controller::create_rate_handler,
        rate_controller::get_rate_handler,
        rate_controller::update_rate_handler,
        rate_controller::delete_rate_handler,
        
        // OAuth controller paths
  
     
    ),
    components(
        schemas(
            // Auth schemas
            auth_controller::LoginRequest,
            auth_controller::RegisterRequest,
            
            // Rate schemas
            rate_controller::CreateRateRequest,
            rate_controller::UpdateRateRequest,
            rate_controller::RateResponse,
            

        )
    ),
    info(
        title = "My Rust-Engine API",
        version = "1.0.0",
        description = "API documentation for the Rust-Engine application"
    )
)]
pub struct ApiDoc;
