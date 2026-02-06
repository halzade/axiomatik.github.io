use axiomatik_web::db::database;
use axiomatik_web::db::database::SurrealError;
use axiomatik_web::system::commands::{create_user, delete_user, CommandError};
use axiomatik_web::system::configuration::ConfigurationError;
use axiomatik_web::system::server;
use axiomatik_web::system::server::ServerError;
use axiomatik_web::system::{configuration, heartbeat, logger};
use fs::create_dir_all;
use std::env;
use std::fs;
use std::io::Error;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("configuration error")]
    ApplicationConfiguration(#[from] ConfigurationError),

    #[error("io error")]
    ApplicationIo(#[from] Error),

    #[error("command error")]
    ApplicationCommand(#[from] CommandError),

    #[error("sureal error")]
    ApplicationSurreal(#[from] SurrealError),

    #[error("server error")]
    ApplicationServerError(#[from] ServerError),
}

// TODO X try, crate: validator
// TODO X nejsou vyřešeny státní svátky

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
        delete_user(&args).await?;
    }

    if args.len() > 0 {
        // TODO X stop
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
    create_dir_all("web/u")?;

    /*
     * Start regular actions
     */
    info!("startup actions");
    heartbeat::heart_beat();

    /*
     * Database
     */
    database::initialize_database().await?;

    /*
     * Routers
     * - application router
     * - web router
     */
    let router_app = server.start_app_server().await?;
    let router_web = server.start_web_server().await?;
    server.status_start()?;

    let config = configuration::get_config()?;
    let addr_app = format!("{}:{}", config.host, config.port_app);
    let addr_web = format!("{}:{}", config.host, config.port_web);

    /*
     * Listeners
     */
    let app_listener = TcpListener::bind(&addr_app).await?;
    info!("listening on {}", addr_app);

    let web_listener = TcpListener::bind(&addr_web).await?;
    info!("listening on {}", addr_web);

    /*
     * Start Application
     */
    axum::serve(app_listener, router_app).await?;
    axum::serve(web_listener, router_web).await?;

    info!("end.");
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn text_context_load() {
        // the smokiest test
    }
}
