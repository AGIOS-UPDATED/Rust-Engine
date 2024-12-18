use actix_web::web;
use crate::app::controllers::oauth_controller::{
    google_oauth_handler, google_oauth_callback_handler,
    slack_oauth_handler, slack_oauth_callback_handler,
    github_oauth_handler, github_oauth_callback_handler,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/oauth")
            .route("/google", web::get().to(google_oauth_handler))
            .route("/google/callback", web::get().to(google_oauth_callback_handler))
            .route("/slack", web::get().to(slack_oauth_handler))
            .route("/slack/callback", web::get().to(slack_oauth_callback_handler))
            .route("/github", web::get().to(github_oauth_handler))
            .route("/github/callback", web::get().to(github_oauth_callback_handler))
    );
}
