use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::config::{decode_jwt, Config};

pub async fn jwt_middleware(
    req: ServiceRequest,
    auth: BearerAuth,
    config: web::Data<Config>,
) -> Result<ServiceRequest, Error> {
    let token = auth.token();
    match decode_jwt(token, &config.jwt_secret) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}
