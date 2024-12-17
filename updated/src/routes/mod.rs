use actix_web::web;
use crate::controllers::auth::{sign_in, sign_up};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(sign_up))
            .route("/signin", web::post().to(sign_in)),
    );
}
