use crate::api;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api/orders").service(api::api_orders::create));
    cfg.service(
        web::scope("/api/health")
            .service(api::api_health_check::startup)
            .service(api::api_health_check::ready)
            .service(api::api_health_check::live),
    );
    cfg.service(web::scope("/api/info").service(api::api_info::info));
}
