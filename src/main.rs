mod config;
mod entity;
mod errors;
mod services;
mod state;
mod types;
mod utils;

use std::{env, io};

use config::AppConfig;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tonic::transport::Server;
use tracing::{Level, info};

pub mod blazer {
    pub mod message {
        tonic::include_proto!("blazer.message");
    }
    tonic::include_proto!("blazer");
}

use services::account::AccountService;
// use services::health::HealthService;
// use services::message::MessageService;
use state::AppState;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    dotenv().ok();

    let config = AppConfig::new().expect("Failed to load configuration");

    let db = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    let state = AppState::new(db, config.clone());

    let addr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Failed to parse address");

    let account_service = AccountService::new(state.clone());
    // let health_service = HealthService::new();
    // let message_service = MessageService::new(state.clone());

    info!(
        "Starting blazer-backend-{} on {}",
        env!("CARGO_PKG_VERSION"),
        addr
    );

    Server::builder()
        .add_service(blazer::account_service_server::AccountServiceServer::new(account_service))
        // .add_service(blazer::health_service_server::HealthServiceServer::new(health_service))
        // .add_service(blazer::message::message_service_server::MessageServiceServer::new(
        //     message_service,
        // ))
        .serve(addr)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}
