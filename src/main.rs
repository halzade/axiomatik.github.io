use crate::ApplicationError::UnrecognizedParameters;
use axiomatik_web::db::database;
use axiomatik_web::db::database::SurrealError;
use axiomatik_web::db::database_article::DatabaseArticle;
use axiomatik_web::db::database_system::DatabaseSystem;
use axiomatik_web::db::database_user::DatabaseUser;
use axiomatik_web::system::commands::{create_user, delete_user, CommandError};
use axiomatik_web::system::configuration::ConfigurationError;
use axiomatik_web::system::server::{ServerError, TheState};
use axiomatik_web::system::{configuration, heartbeat, logger};
use axiomatik_web::system::{data_system, data_updates, server};
use fs::create_dir_all;
use std::env;
use std::fs;
use std::io::Error;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

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

    #[error("db error")]
    ApplicationDb(#[from] surrealdb::Error),

    #[error("unrecognized parameter")]
    UnrecognizedParameters,
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    /*
     * command arguments if any
     */
    let args: Vec<String> = env::args().collect();

    /*
     * databases
     */
    let surreal = Arc::new(database::init_db_connection().await?);
    let dba = Arc::new(DatabaseArticle::new(surreal.clone()));
    let dbu = Arc::new(DatabaseUser::new(surreal.clone()));
    let dbs = Arc::new(DatabaseSystem::new(surreal.clone()));

    // if there are no articles at all, create the table
    surreal.db.query("DEFINE TABLE article SCHEMALESS;").await?;
    surreal.db.query("DEFINE TABLE article_status SCHEMALESS;").await?;

    /*
     * in memory application data
     */
    let ds = Arc::new(data_system::new());
    let dv = Arc::new(data_updates::new());

    /*
     * the application state
     */
    let state = TheState { dba, dbu, dbs, ds, dv };

    /*
     * process the commands
     */
    if args.len() > 1 && args[1] == "create-user" {
        create_user(&args, &state).await;
    }
    if args.len() > 1 && args[1] == "delete-user" {
        delete_user(&args, &state).await?;
    }

    if args.len() > 1 {
        return Err(UnrecognizedParameters);
    }

    /*
     * server
     */
    let server = server::connect(state).await?;
    if !server.is_off() {
        info!("But the Application has already started");
        info!("Shutting down gracefully...");
        signal::ctrl_c().await.ok();
    }

    /*
     * init application infrastructure
     */
    info!("Application starting...");
    logger::config();
    // the uploads directory
    create_dir_all("web/u")?;

    /*
     * start regular actions
     */
    info!("startup actions");
    heartbeat::heart_beat();

    /*
     * routers
     * - application router
     * - web router
     */
    let app_router = server.start_app_router().await?;
    let web_router = server.start_web_router().await?;
    server.status_start()?;

    let config = configuration::get_config()?;
    let app_address = format!("{}:{}", config.host, config.port_app);
    let web_address = format!("{}:{}", config.host, config.port_web);

    /*
     * listeners
     */
    let app_listener = TcpListener::bind(&app_address).await?;
    info!("listening on {}", app_address);

    let web_listener = TcpListener::bind(&web_address).await?;
    info!("listening on {}", web_address);

    /*
     * start Application
     */
    axum::serve(app_listener, app_router).await?;
    axum::serve(web_listener, web_router).await?;

    info!("end.");
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn text_context_load() {
        // the smokiest test ever
    }
}
