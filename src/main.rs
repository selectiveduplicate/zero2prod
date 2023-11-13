use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::startup::run;

use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration settings");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let connection = PgPool::connect(&configuration.database_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    run(listener, connection)?.await
}
