use crate::api::docs::health_check_api_doc::HealthCheckApiDoc;
use crate::api::docs::order_api_doc::OrderApiDoc;
use utoipa::OpenApi;
use utoipa::openapi::OpenApi as OpenApiStruct;

pub fn open_api_docs() -> OpenApiStruct {
    let mut openapi = OrderApiDoc::openapi();

    openapi.merge(HealthCheckApiDoc::openapi());

    openapi
}
