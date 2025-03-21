use rust_api::configuration::get_configuration;
use rust_api::startup::run;
use rust_api::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rust_api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await?;
    Ok(())
}
