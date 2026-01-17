mod auth;
mod commands;
mod configuration;
mod content_management;
mod content_worker;
mod database;
mod external;
mod form_account;
mod form_change_password;
mod form_login;
mod form_new_article;
mod form_search;
mod library;
mod logger;
mod name_days;
mod server;
mod templates;
mod validation;
mod library_name_days;

use crate::commands::{create_user, delete_user, print_from_db};
use fs::create_dir_all;
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

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

    // TODO terminate if application already running.

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
    info!("startup actions");
    content_management::update_index_date();
    content_management::update_index_nameday();
    content_management::update_index_weather().await;

    /*
     * Start regular actions
     */
    content_worker::heart_beat();
    content_worker::midnight_worker();
    content_worker::weather_worker();

    /*
     * Database
     */
    let db = Arc::new(database::init_db().await);

    /*
     * Server
     */
    let router = server::router(db);
    let addr = format!("{}:{}", config.application.host, config.application.port);
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
