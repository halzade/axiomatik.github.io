use axiomatik_web::system::commands::{create_user, delete_user};
use axiomatik_web::system::configuration;
use axiomatik_web::content_worker;
use axiomatik_web::logger;
use axiomatik_web::system::server;
use fs::create_dir_all;
use std::env;
use std::fs;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info};
use axiomatik_web::db::database;

#[tokio::main]
async fn main() {
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

    if server::is_started().await {
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
    create_dir_all("u").unwrap();

    /*
     * Trigger actions at startup
     */
    info!("startup actions data");

    /*
     * Start regular actions
     */
    info!("startup actions workers");
    content_worker::heart_beat();
    content_worker::midnight_worker();
    content_worker::weather_worker();

    /*
     * Database
     */
    database::initialize_database().await;

    /*
     * Server
     */
    let router = server::start_router().await;
    let config = configuration::get_config().expect("Failed to read configuration.");
    let addr = format!("{}:{}", config.host, config.port);
    info!("listening on {}", addr);
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
}
