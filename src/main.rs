use axiomatik_web::configuration::get_configuration;
use axiomatik_web::db;
use chrono::{Datelike, Local, Weekday};
use std::env;
use std::fs;
use std::sync::Arc;
use log::trace;
use tokio::time::interval;
use tokio::time::{self, Duration, Instant};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use reqwest;
use serde_json;

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

    /*
     *  trigger actions at startup
     *
     *  in devel,
     *  changing files will cause application restart, because of cargo watch
     */
    info!("startup actions");
    update_index_date();
    update_index_weather().await;

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

    info!("start heart beat");
    heart_beat();

    // scheduled actions
    info!("schedule midnight worker");
    midnight_worker();
    weather_worker();

    // start the application
    if let Err(err) = axum::serve(listener, app).await {
        error!("axum server exited: {:?}", err);
    };

    info!("end.");
}

fn heart_beat() {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            trace!("beat");
        }
    });
}

fn midnight_worker() {
    tokio::spawn(async {
        let start = next_midnight_instant();
        let mut interval = time::interval_at(start, Duration::from_secs(24 * 60 * 60));

        loop {
            interval.tick().await;
            info!("midnight event");
            update_index_date();
        }
    });
}

fn weather_worker() {
    tokio::spawn(async {
        // Every 60 minutes
        let mut interval = interval(Duration::from_secs(60 * 60));
        loop {
            interval.tick().await;
            update_index_weather().await;
        }
    });
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

            if content != new_content {
                if let Err(e) = fs::write("index.html", new_content) {
                    eprintln!("Failed to write index.html: {}", e);
                } else {
                    info!("index.html date updated to: {}", date_string);
                }
            } else {
                info!("index.html date is already up to date: {}", date_string);
            }
        }
    }
}

async fn update_index_weather() {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=50.0755&longitude=14.4378&current_weather=true&timezone=Europe/Prague";
    
    let weather_string = match fetch_weather(url).await {
        Ok(temp) => format!("{:.0}°C | Prague", temp),
        Err(_) => "".to_string(),
    };

    if let Ok(content) = fs::read_to_string("index.html") {
        let start_tag = "<!-- WEATHER -->";
        let end_tag = "<!-- /WEATHER -->";

        if let (Some(_start), Some(_end)) = (content.find(start_tag), content.find(end_tag)) {
            let new_content = replace_weather_in_content(&content, &weather_string);

            if content != new_content {
                if let Err(e) = fs::write("index.html", new_content) {
                    eprintln!("Failed to write index.html for weather: {}", e);
                } else {
                    info!("index.html weather updated to: {}", weather_string);
                }
            } else {
                info!("index.html weather is already up to date: {}", weather_string);
            }
        }
    }
}

fn replace_weather_in_content(content: &str, weather_string: &str) -> String {
    let start_tag = "<!-- WEATHER -->";
    let end_tag = "<!-- /WEATHER -->";

    if let (Some(start), Some(end)) = (content.find(start_tag), content.find(end_tag)) {
        let mut new_content = content[..start + start_tag.len()].to_string();
        new_content.push_str(weather_string);
        new_content.push_str(&content[end..]);
        new_content
    } else {
        content.to_string()
    }
}

async fn fetch_weather(url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let temp = resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or("Could not find temperature")?;
    Ok(temp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_replacement_logic() {
        let content = "<html><!-- WEATHER -->OLD<!-- /WEATHER --></html>";
        let weather_string = "23°C | Prague";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(new_content, "<html><!-- WEATHER -->23°C | Prague<!-- /WEATHER --></html>");
    }

    #[test]
    fn test_no_weather_replacement_if_same() {
        let content = "<html><!-- WEATHER -->23°C | Prague<!-- /WEATHER --></html>";
        let weather_string = "23°C | Prague";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(content, new_content);
    }

    #[test]
    fn test_weather_replacement_empty_if_exception() {
        let content = "<html><!-- WEATHER -->23°C | Prague<!-- /WEATHER --></html>";
        let weather_string = "";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(new_content, "<html><!-- WEATHER --><!-- /WEATHER --></html>");
    }
}
