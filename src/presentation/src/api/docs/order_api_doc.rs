use crate::api::api_orders::__path_create;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Order", description = "Order management endpoints.")
    ),
    paths(
        create,
    )
)]
pub(crate) struct OrderApiDoc;
