use axiomatik_web::configuration::get_configuration;
use axiomatik_web::db;
use chrono::{Datelike, Local, Weekday};
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::time::interval;
use tokio::time::{self, Duration, Instant};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "create-user" {
        if args.len() != 4 {
            eprintln!("Usage: cargo run -- create-user <username> <password>");
            std::process::exit(1);
        }

        let username = &args[2];
        let password = &args[3];

        let db = db::init_db().await;
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

        let db = db::init_db().await;
        match db.delete_user(username).await {
            Ok(_) => {
                println!("User '{}' attempted to be deleted.", username);
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

    let db = Arc::new(db::init_db().await);

    let app = axiomatik_web::app(db);

    let addr = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {}", addr));


    info!("start heartbeat");
    // start heart beat
    let _hb = heart_beat();

    // start the application
    if let Err(err) = axum::serve(listener, app).await {
        error!("axum server exited: {:?}", err);
    };


    // scheduled actions
    // let _ = midnight_worker();

    // trigger actions at startup
    // let _ = update_index_date();
    info!("end.");
}

fn heart_beat() -> tokio::task::JoinHandle<()> {
    // heart beat
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            info!("beat");
        }
    })
}

fn midnight_worker() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        let start = next_midnight_instant();
        let mut interval = time::interval_at(start, Duration::from_secs(24 * 60 * 60));

        loop {
            interval.tick().await;
            info!("midnight event");
            update_index_date();
        }
    })
}

fn next_midnight_instant() -> Instant {
    let now = Local::now();

    let next_midnight = now
        .date_naive()
        .succ_opt()
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let duration_until = (next_midnight - now.naive_local()).to_std().unwrap();

    Instant::now() + duration_until
}

fn update_index_date() {
    let now = Local::now();
    let day_name = match now.weekday() {
        Weekday::Mon => "Pondělí",
        Weekday::Tue => "Úterý",
        Weekday::Wed => "Středa",
        Weekday::Thu => "Čtvrtek",
        Weekday::Fri => "Pátek",
        Weekday::Sat => "Sobota",
        Weekday::Sun => "Neděle",
    };

    let month_name = match now.month() {
        1 => "ledna",
        2 => "února",
        3 => "března",
        4 => "dubna",
        5 => "května",
        6 => "června",
        7 => "července",
        8 => "srpna",
        9 => "září",
        10 => "října",
        11 => "listopadu",
        12 => "prosince",
        _ => unreachable!(),
    };

    let date_string = format!("{} {}. {} {}", day_name, now.day(), month_name, now.year());

    if let Ok(content) = fs::read_to_string("index.html") {
        let start_tag = "<!-- DATE -->";
        let end_tag = "<!-- /DATE -->";

        if let (Some(start), Some(end)) = (content.find(start_tag), content.find(end_tag)) {
            let mut new_content = content[..start + start_tag.len()].to_string();
            new_content.push_str(&date_string);
            new_content.push_str(&content[end..]);

            if let Err(e) = fs::write("index.html", new_content) {
                eprintln!("Failed to write index.html: {}", e);
            } else {
                info!("index.html date updated to: {}", date_string);
            }
        }
    }
}
