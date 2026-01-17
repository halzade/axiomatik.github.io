use axiomatik_web::commands::{create_user, delete_user, print_from_db};
use fs::create_dir_all;
use std::env;
use std::fs;
use tokio::net::TcpListener;
use tracing::{error, info};
use axiomatik_web::{configuration, content_management, content_worker, database, logger, server};

#[tokio::main]
async fn main() {
    let config = configuration::get_config().expect("Failed to read configuration.");
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
    if args.len() > 1 && args[1] == "print-from-db" {
        print_from_db(&args).await;
    }

    // TODO terminate if application already running

    /*
     * Init Application Infrastructure
     */
    info!("Application starting...");
    logger::config();
    create_dir_all("uploads").unwrap();
    create_dir_all("snippets").unwrap();

    /*
     * Trigger actions at startup
     */
    let now = chrono::Local::now();
    info!("startup actions");
    content_management::update_all_header_info(now).await;

    /*
     * Start regular actions
     */
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
    let router = server::router();
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
