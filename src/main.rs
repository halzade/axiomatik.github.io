use crate::ApplicationError::UnrecognizedParameters;
use axiomatik_web::db::database;
use axiomatik_web::db::database::SurrealError;
use axiomatik_web::db::database_article::DatabaseArticle;
use axiomatik_web::db::database_system::DatabaseSystem;
use axiomatik_web::db::database_user::{DatabaseUser, SurrealUserError};
use axiomatik_web::system::commands::{create_admin_user, CommandError};
use axiomatik_web::system::configuration::ConfigurationError;
use axiomatik_web::system::server::{ServerError, TheState};
use axiomatik_web::system::{configuration, logger};
use axiomatik_web::system::{data_system, data_updates, server};
use axiomatik_web::worker::heartbeat;
use axiomatik_web::worker::midnight_worker::{start_midnight_worker, MidnightWorkerError};
use axiomatik_web::worker::weather_worker::{start_weather_worker, WeatherWorkerError};
use fs::create_dir_all;
use std::env;
use std::fs;
use std::io::Error;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info, warn};

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

    #[error("midnight error")]
    Midnight(#[from] MidnightWorkerError),

    #[error("weather error")]
    Weather(#[from] WeatherWorkerError),

    #[error("surreal user error")]
    SurrealUser(#[from] SurrealUserError),
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    /*
     * command arguments if any
     */
    let args: Vec<String> = env::args().collect();
    error!("[{:?}]", args);

    if args.len() > 1 {
        return Err(UnrecognizedParameters);
    }

    /*
     * databases
     */
    let surreal = Arc::new(database::init_db_connection().await?);
    let dba = Arc::new(DatabaseArticle::new(surreal.clone()));
    let dbu = Arc::new(DatabaseUser::new(surreal.clone()));
    let dbs = Arc::new(DatabaseSystem::new(surreal.clone()));

    /*
     * in memory application data
     */
    let ds = Arc::new(data_system::new());
    let dv = Arc::new(data_updates::new());

    /*
     * the application state
     */
    let config = configuration::get_config()?;
    #[rustfmt::skip]
    let state = TheState {
        dba, dbu, dbs, ds, dv,
        start_time: chrono::Utc::now(),
        config: config.clone(),
    };

    if !state.dbu.admin_exists().await? {
        warn!("no admin found");

        // create bootstrap admin if none exists
        create_admin_user(&state).await?;
    }

    /*
     * server
     */
    let server = server::connect(&state).await?;
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
    heartbeat::start_heart_beat();
    start_weather_worker(state.clone())?;
    start_midnight_worker(state.clone())?;

    /*
     * routers
     * - application router
     * - web router
     */
    let app_router = server.start_app_router().await?;
    let web_router = server.start_web_router().await?;
    server.status_start()?;

    let app_address = format!("{}:{}", config.host, config.port.app);
    let web_address = format!("{}:{}", config.host, config.port.web);

    info!("Application starting...");
    info!("config app_port: {}", config.port.app);
    info!("config web_port: {}", config.port.web);

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
    let app_server = axum::serve(app_listener, app_router);
    let web_server = axum::serve(web_listener, web_router);

    // execute web future and app future concurrently
    tokio::try_join!(app_server, web_server)?;

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
