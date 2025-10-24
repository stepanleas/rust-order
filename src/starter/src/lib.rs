use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;
use application::{CustomerMessageListenerImpl, Settings};
use infrastructure::{DbPool, PostgresCustomerRepository, PostgresOrderRepository};
use kafka::client::KafkaClient;
use log::{debug, error, info};
use presentation::AppState;
use std::sync::Arc;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run() -> Result<Server> {
    let settings = Settings::default().load()?;
    run_internal(&settings).await
}

async fn run_internal(settings: &Settings) -> Result<Server> {
    info!("Starting HTTP server at {}", settings.http_url);
    debug!("with configuration: {:?}", settings);

    let pool = infrastructure::configure(settings).await?;

    let app_state = AppState {
        settings: settings.clone(),
        order_repository: Arc::new(PostgresOrderRepository::new(&pool)),
        customer_repository: Arc::new(PostgresCustomerRepository::new(&pool)),
    };

    listen_to_kafka(settings.clone(), pool.clone());

    let server = HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(presentation::open_api_docs())
            .map(|app| app.wrap(Logger::default()))
            .map(|app| app.configure(presentation::configure))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .app_data(Data::new(app_state.clone()))
            .into_app()
    })
    .bind(&settings.http_url)?
    .run();

    Ok(server)
}

fn listen_to_kafka(settings: Settings, pool: DbPool) {
    std::thread::spawn(move || {
        let mut kafka_client = KafkaClient::new(vec![settings.kafka_host.to_owned()]);
        if let Err(e) = kafka_client.load_metadata_all() {
            error!("Failed to load Kafka metadata: {}", e);
        }

        if let Err(e) = messaging::listen(
            kafka_client,
            Arc::new(CustomerMessageListenerImpl::new(Arc::new(
                PostgresCustomerRepository::new(&pool),
            ))),
        ) {
            error!("Kafka listener stopped: {}", e);
        }
    });
}
