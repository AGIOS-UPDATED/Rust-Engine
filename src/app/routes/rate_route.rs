use actix_web::web;
use crate::app::controllers::rate_controller::{
    create_rate_handler, get_rate_handler, update_rate_handler, delete_rate_handler
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            // Correct the route definitions: use `.to(handler)` not `.post(handler)`
            .route("/rates", web::post().to(create_rate_handler))
            .route("/rates/{id}", web::get().to(get_rate_handler))
            .route("/rates/{id}", web::put().to(update_rate_handler))
            .route("/rates/{id}", web::delete().to(delete_rate_handler))
    );
}
