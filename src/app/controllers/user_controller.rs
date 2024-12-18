use actix_web::{HttpResponse, Responder, web};
use postgrest::Postgrest;
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;

/// Login request body schema
#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Register request body schema
#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

/// Login a user
///
/// This endpoint allows a user to login with an email and password.
/// The request body expects JSON matching `LoginRequest`.
///
/// Response:
/// - 200: Login successful
/// - 500: Internal Server Error
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn login_handler(
    supabase: web::Data<Arc<Postgrest>>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let email = &req.email;
    let response = supabase
        .from("users")
        .select("*")
        .eq("email", email)
        .execute()
        .await;

    match response {
        Ok(_) => HttpResponse::Ok().body(format!("Login successful for {}", email)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// Logout a user
///
/// Invalidate the user's session or token.
///
/// Response:
/// - 200: Logged out
/// - 500: Internal Server Error
#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Logged out", body = String),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn logout_handler(_supabase: web::Data<Arc<Postgrest>>) -> impl Responder {
    HttpResponse::Ok().body("Logged out (stub)")
}

/// Register a new user
///
/// Create a new user account with email and password.
/// The request body expects JSON matching `RegisterRequest`.
///
/// Response:
/// - 200: User registered
/// - 500: Internal Server Error
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered", body = String),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn register_handler(
    supabase: web::Data<Arc<Postgrest>>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let email = &req.email;
    let password = &req.password;

    // Convert the JSON value into a String before inserting.
    let body_json = serde_json::json!([{
        "email": email,
        "password": password
    }]);
    let body_str = serde_json::to_string(&body_json).unwrap();

    let response = supabase
        .from("users")
        .insert(body_str)
        .execute()
        .await;

    match response {
        Ok(_) => HttpResponse::Ok().body(format!("User registered: {}", email)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
