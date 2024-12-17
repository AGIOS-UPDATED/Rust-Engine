use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use supabase::{Client, Supabase};
use std::env;

// Struct to represent the user in the database
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub full_name: Option<String>,
}

// Initialize Supabase client
fn get_supabase_client() -> Client {
    let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");
    Client::new(&url, &key)
}

// CREATE: Register a new user
pub async fn create_user(user: web::Json<User>) -> impl Responder {
    let client = get_supabase_client();

    // Insert user data into Supabase
    let response = client
        .from("users")
        .insert(vec![user.into_inner()])
        .execute()
        .await;

    match response {
        Ok(_) => HttpResponse::Created().json(user.into_inner()),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

// READ: Get a user by email
pub async fn get_user(email: web::Path<String>) -> impl Responder {
    let client = get_supabase_client();

    // Query user from Supabase
    let response = client
        .from("users")
        .eq("email", email.as_str())
        .select("*")
        .execute()
        .await;

    match response {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

// UPDATE: Update user information
pub async fn update_user(
    email: web::Path<String>,
    user: web::Json<User>,
) -> impl Responder {
    let client = get_supabase_client();

    // Update user in Supabase
    let response = client
        .from("users")
        .eq("email", email.as_str())
        .update(user.into_inner())
        .execute()
        .await;

    match response {
        Ok(_) => HttpResponse::Ok().json(user.into_inner()),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}

// DELETE: Delete user by email
pub async fn delete_user(email: web::Path<String>) -> impl Responder {
    let client = get_supabase_client();

    // Delete user from Supabase
    let response = client
        .from("users")
        .eq("email", email.as_str())
        .delete()
        .execute()
        .await;

    match response {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting user"),
    }
}
