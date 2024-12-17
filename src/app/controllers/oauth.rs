use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct OAuthCallback {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct SlackTokenResponse {
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct FacebookTokenResponse {
    access_token: String,
}

async fn exchange_code_for_token(client_id: &str, client_secret: &str, redirect_uri: &str, code: &str, token_url: &str) -> Result<String, String> {
    let client = Client::new();
    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("code", code),
        ("grant_type", "authorization_code"),
    ];

    let response = client
        .post(token_url)
        .form(&params)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let response_json: serde_json::Value = response.json().await.map_err(|err| err.to_string())?;
    response_json["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("Failed to retrieve access token".to_string())
}

pub async fn google_callback(params: web::Query<OAuthCallback>) -> impl Responder {
    let google_client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let google_redirect_uri = env::var("GOOGLE_REDIRECT_URI").unwrap();
    let google_token_url = "https://oauth2.googleapis.com/token";

    match exchange_code_for_token(&google_client_id, &google_client_secret, &google_redirect_uri, &params.code, google_token_url).await {
        Ok(access_token) => HttpResponse::Ok().json(access_token),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn slack_callback(params: web::Query<OAuthCallback>) -> impl Responder {
    let slack_client_id = env::var("SLACK_CLIENT_ID").unwrap();
    let slack_client_secret = env::var("SLACK_CLIENT_SECRET").unwrap();
    let slack_redirect_uri = env::var("SLACK_REDIRECT_URI").unwrap();
    let slack_token_url = "https://slack.com/api/oauth.v2.access";

    match exchange_code_for_token(&slack_client_id, &slack_client_secret, &slack_redirect_uri, &params.code, slack_token_url).await {
        Ok(access_token) => HttpResponse::Ok().json(access_token),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn facebook_callback(params: web::Query<OAuthCallback>) -> impl Responder {
    let facebook_client_id = env::var("FACEBOOK_CLIENT_ID").unwrap();
    let facebook_client_secret = env::var("FACEBOOK_CLIENT_SECRET").unwrap();
    let facebook_redirect_uri = env::var("FACEBOOK_REDIRECT_URI").unwrap();
    let facebook_token_url = "https://graph.facebook.com/v11.0/oauth/access_token";

    match exchange_code_for_token(&facebook_client_id, &facebook_client_secret, &facebook_redirect_uri, &params.code, facebook_token_url).await {
        Ok(access_token) => HttpResponse::Ok().json(access_token),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
