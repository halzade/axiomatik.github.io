use axiomatik_web::db;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "create-user" {
        if args.len() != 4 {
            eprintln!("Usage: cargo run -- create-user <username> <password>");
            std::process::exit(1);
        }

        let username = &args[2];
        let password = &args[3];

        let db = db::init_db().await.expect("Failed to initialize database");
        match axiomatik_web::auth::create_editor_user(&db, username, password).await {
            Ok(_) => {
                println!("Editor user '{}' created successfully.", username);
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Failed to create editor user: {}", e);
                std::process::exit(1);
            }
        }
    }

    if args.len() > 1 && args[1] == "delete-user" {
        if args.len() != 3 {
            eprintln!("Usage: cargo run -- delete-user <username>");
            std::process::exit(1);
        }

        let username = &args[2];

        let db = db::init_db().await.expect("Failed to initialize database");
        match db.delete_user(username).await {
            Ok(Some(_)) => {
                println!("User '{}' deleted successfully.", username);
                std::process::exit(0);
            }
            Ok(None) => {
                println!("User '{}' not found.", username);
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Failed to delete user: {}", e);
                std::process::exit(1);
            }
        }
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Application starting...");

    // Ensure uploads directory exists
    fs::create_dir_all("uploads").unwrap();
    fs::create_dir_all("snippets").unwrap();

    let db = Arc::new(db::init_db().await.expect("Failed to initialize database"));

    let app = axiomatik_web::app(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
