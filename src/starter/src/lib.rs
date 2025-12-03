use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Result;
use application::ports::input::message::customer_message_listener::ApplicationCustomerMessageListener;
use application::ports::input::message::product_message_listener::ApplicationProductMessageListener;
use application::settings::Settings;
use infrastructure::DbPool;
use infrastructure::postgres_customer_repository::PostgresCustomerRepository;
use infrastructure::postgres_order_repository::PostgresOrderRepository;
use infrastructure::postgres_product_repository::PostgresProductRepository;
use kafka::client::KafkaClient;
use messaging::event_handlers::KafkaEventHandlerFactory;
use presentation::app_state::AppState;
use presentation::middleware::correlation_id::CorrelationIdMiddleware;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run() -> Result<Server> {
    let settings = Settings::default().load()?;
    run_internal(&settings).await
}

async fn run_internal(settings: &Settings) -> Result<Server> {
    tracing::info!("Starting HTTP server at {}", settings.http_url);
    tracing::debug!("with configuration: {:?}", settings);

    let pool = infrastructure::config::configure(settings.database_url.clone()).await?;

    let app_state = AppState {
        settings: settings.clone(),
        order_repository: Arc::new(PostgresOrderRepository::new(&pool)),
        customer_repository: Arc::new(PostgresCustomerRepository::new(&pool)),
        product_repository: Arc::new(PostgresProductRepository::new(&pool)),
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(CorrelationIdMiddleware)
            .into_utoipa_app()
            .openapi(presentation::api::docs::open_api_docs())
            .map(|app| app.wrap(Logger::default()))
            .map(|app| app.configure(presentation::config::configure))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .app_data(Data::new(app_state.clone()))
            .into_app()
    })
    .bind(&settings.http_url)?
    .run();

    listen_to_kafka(settings.clone(), pool.clone());

    Ok(server)
}

fn listen_to_kafka(settings: Settings, pool: DbPool) {
    let mut kafka_client = KafkaClient::new(vec![settings.kafka_host.clone()]);
    if let Err(e) = kafka_client.load_metadata_all() {
        tracing::error!("Failed to load Kafka metadata: {}", e);
    }

    std::thread::spawn(move || {
        let customer_listener = Arc::new(ApplicationCustomerMessageListener::new(Arc::new(
            PostgresCustomerRepository::new(&pool),
        )));
        let product_listener = Arc::new(ApplicationProductMessageListener::new(Arc::new(
            PostgresProductRepository::new(&pool),
        )));

        let factory = KafkaEventHandlerFactory::new(customer_listener, product_listener);

        messaging::listen(kafka_client, factory, "order-service-group".to_string())
    });
}
