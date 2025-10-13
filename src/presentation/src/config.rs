use crate::api;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api/orders").service(api::create_order));
    cfg.service(
        web::scope("/api/health")
            .service(api::startup)
            .service(api::ready)
            .service(api::live),
    );
    cfg.service(web::scope("/api/info").service(api::info));
}
