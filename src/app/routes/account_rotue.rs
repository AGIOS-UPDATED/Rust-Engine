use actix_web::web;
use crate::app::controllers::account_controller::{login_handler, logout_handler, register_handler};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login_handler))
            .route("/logout", web::post().to(logout_handler))
            .route("/register", web::post().to(register_handler)),
    );
}
