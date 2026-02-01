use axiomatik_web::db::database;
use axiomatik_web::logger;
use axiomatik_web::system::commands::{create_user, delete_user};
use axiomatik_web::system::configuration::ConfigurationError;
use axiomatik_web::system::server;
use axiomatik_web::system::{configuration, heartbeat};
use fs::create_dir_all;
use std::env;
use std::fs;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("configuration error")]
    Configuration(#[from] ConfigurationError),

    #[error("io error")]
    Io(#[from] std::io::Error),
}

// TODO X authorization framework, crate axum-login, axum_gate has OAuth2
// TODO X try, crate: validator
// TODO X Proper test framework
// TODO X Nejsou vyřešeny státní svátky

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    /*
     * Command arguments if any
     */
    let args: Vec<String> = env::args().collect();

    /*
     * Process Commands
     */
    if args.len() > 1 && args[1] == "create-user" {
        create_user(&args).await;
    }
    if args.len() > 1 && args[1] == "delete-user" {
        delete_user(&args).await;
    }

    /*
     * Server
     */
    let server = server::new();
    if !server.is_off() {
        info!("But the Application has already started");
        info!("Shutting down gracefully...");
        signal::ctrl_c().await.ok();
    }

    /*
     * Init Application Infrastructure
     */
    info!("Application starting...");
    logger::config();

    // the uploads directory
    create_dir_all("u")?;

    /*
     * Start regular actions
     */
    info!("startup actions");
    heartbeat::heart_beat();

    /*
     * Database
     */
    database::initialize_database().await;

    /*
     * Router
     */
    let router = server.start_server().await;

    let config = configuration::get_config()?;
    let addr = format!("{}:{}", config.host, config.port);
    info!("listening on {}", addr);

    /*
     * Listener
     */
    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {}", addr));

    /*
     * Start Application
     */
    if let Err(err) = axum::serve(listener, router).await {
        error!("axum server exited: {:?}", err);
    };

    info!("end.");
    Ok(())
}
