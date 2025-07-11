use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_string = configuration.database.connection_string();
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to DB");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration.application_port))?;

    run(listener, pool)?.await
}
