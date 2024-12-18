use actix_web::{HttpResponse, Responder, web};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

/// Request body for creating a rate
#[derive(Deserialize, ToSchema)]
pub struct CreateRateRequest {
    pub name: String,
    pub value: f64,
}

/// Request body for updating a rate
#[derive(Deserialize, ToSchema)]
pub struct UpdateRateRequest {
    pub value: f64,
}

/// Response body for a rate
#[derive(Serialize, ToSchema)]
pub struct RateResponse {
    pub id: i64,
    pub name: String,
    pub value: f64,
}

/// Create a new rate
#[utoipa::path(
    post,
    path = "/rates",
    request_body = CreateRateRequest,
    responses(
        (status = 201, description = "Rate created", body = RateResponse),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn create_rate_handler(
    supabase: web::Data<Arc<Postgrest>>,
    req: web::Json<CreateRateRequest>,
) -> impl Responder {
    // Handler code from previous examples...
    HttpResponse::Created().json(RateResponse { id: 1, name: req.name.clone(), value: req.value })
}

/// Update an existing rate
#[utoipa::path(
    put,
    path = "/rates/{id}",
    request_body = UpdateRateRequest,
    params(
        ("id" = i64, Path, description = "ID of the rate to update")
    ),
    responses(
        (status = 200, description = "Rate updated", body = RateResponse),
        (status = 404, description = "Rate not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn update_rate_handler(
    supabase: web::Data<Arc<Postgrest>>,
    path: web::Path<i64>,
    req: web::Json<UpdateRateRequest>,
) -> impl Responder {
    // Handler code...
    HttpResponse::Ok().json(RateResponse { id: path.into_inner(), name: "Updated Rate".to_string(), value: req.value })
}

/// Delete an existing rate
#[utoipa::path(
    delete,
    path = "/rates/{id}",
    params(
        ("id" = i64, Path, description = "ID of the rate to delete")
    ),
    responses(
        (status = 200, description = "Rate deleted"),
        (status = 404, description = "Rate not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn delete_rate_handler(
    supabase: web::Data<Arc<Postgrest>>,
    path: web::Path<i64>,
) -> impl Responder {
    // Handler code...
    HttpResponse::Ok().body(format!("Rate with id {} deleted", path.into_inner()))
}

/// Get an existing rate
#[utoipa::path(
    get,
    path = "/rates/{id}",
    params(
        ("id" = i64, Path, description = "ID of the rate to retrieve")
    ),
    responses(
        (status = 200, description = "Rate retrieved", body = RateResponse),
        (status = 404, description = "Rate not found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn get_rate_handler(
    supabase: web::Data<Arc<Postgrest>>,
    path: web::Path<i64>,
) -> impl Responder {
    // Handler code...
    HttpResponse::Ok().json(RateResponse { id: path.into_inner(), name: "Found Rate".to_string(), value: 123.45 })
}
