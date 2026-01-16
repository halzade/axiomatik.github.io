use axiomatik_web::configuration::get_config;
use axiomatik_web::db;
use fs::create_dir_all;
mod commands;
mod content_management;
mod content_worker;
mod external;
mod library;
mod logger;
mod name_days;
mod server;
mod templates;
pub mod validation;

use crate::commands::{create_user, delete_user, print_from_db};
use chrono::{Datelike, Local, Weekday};
use reqwest;
use serde_json;
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::time::interval;
use tokio::time::{self, Duration, Instant};
use tracing::{error, info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let config = get_config().expect("Failed to read configuration.");
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
    let db = Arc::new(db::init_db().await);

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
