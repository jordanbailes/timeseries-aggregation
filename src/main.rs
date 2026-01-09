use timeseries_aggregation_lib::api::api;
use timeseries_aggregation_lib::database::PostgresRegistry;
use tracing::{info, warn};

mod schema;

const DATABASE_URL_ENV_KEY: &str = "TIMESERIES_AGGREGATION_DATABASE_URL";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    let connection_string = std::env::var(DATABASE_URL_ENV_KEY)
        .unwrap_or_else(|_| panic!("{DATABASE_URL_ENV_KEY} environment variable is not populated. This is required for the service to connect to the database and function."));

    let postgres_registry: PostgresRegistry = PostgresRegistry::new(connection_string);

    postgres_registry.perform_migrations();

    if let Err(e) = postgres_registry.populate_database_with_test_data() {
        warn!("Failed to populate database with test data: {e}")
    }

    info!("Starting API.");

    api(postgres_registry).await.expect("Failed to start API");
}
