use axiomatik_web::configuration::get_configuration;
use axiomatik_web::db;
mod namedays;
use chrono::{Datelike, Local, Weekday};
use reqwest;
use serde_json;
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::time::interval;
use tokio::time::{self, Duration, Instant};
use tracing::{error, info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "create-user" {
        if args.len() != 4 {
            info!("Usage: cargo run -- create-user <username> <password>");
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
                error!("Failed to create editor user: {}", e);
                std::process::exit(1);
            }
        }
    }

    if args.len() > 1 && args[1] == "delete-user" {
        if args.len() != 3 {
            info!("Usage: cargo run -- delete-user <username>");
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
                error!("Failed to delete user: {}", e);
                std::process::exit(1);
            }
        }
    }

    if args.len() > 1 && args[1] == "print-from-db" {
        if args.len() != 3 {
            info!("Usage: cargo run -- print-from-db \"<query>\"");

            for arg in &args {
                println!("Argument: {}", arg);
            }

            std::process::exit(1);
        }

        let query = &args[2];

        let db = db::init_db().await;
        match axiomatik_web::db_tool::print_from_db(&db, query).await {
            Ok(_) => {
                std::process::exit(0);
            }
            Err(e) => {
                error!("Failed to execute query: {}", e);
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
     *  changing files will cause the application to restart, because of cargo watch
     */
    info!("startup actions");
    update_index_date();
    update_index_nameday();
    update_index_weather().await;

    // Ensure the uploads directory exists
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
            update_index_nameday();
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

    let month_name = axiomatik_web::get_czech_month_genitive(now.month());

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
                    error!("Failed to write index.html: {}", e);
                } else {
                    info!("index.html date updated to: {}", date_string);
                }
            } else {
                info!("index.html date is already up to date: {}", date_string);
            }
        }
    }
}

fn update_index_nameday() {
    let name = namedays::today_name_day();
    let nameday_string =
        if name.is_empty() || name.contains("No nameday") || name.contains("Invalid") {
            "".to_string()
        } else {
            format!("Svátek má {}", name)
        };

    if let Ok(content) = fs::read_to_string("index.html") {
        let start_tag = "<!-- NAME_DAY -->";
        let end_tag = "<!-- /NAME_DAY -->";

        if let (Some(_start), Some(_end)) = (content.find(start_tag), content.find(end_tag)) {
            let new_content = replace_nameday_in_content(&content, &nameday_string);

            if content != new_content {
                if let Err(e) = fs::write("index.html", new_content) {
                    error!("Failed to write index.html for nameday: {}", e);
                } else {
                    info!("index.html nameday updated to: {}", nameday_string);
                }
            } else {
                info!(
                    "index.html nameday is already up to date: {}",
                    nameday_string
                );
            }
        }
    }
}

fn replace_nameday_in_content(content: &str, nameday_string: &str) -> String {
    let start_tag = "<!-- NAME_DAY -->";
    let end_tag = "<!-- /NAME_DAY -->";
    replace_in_content(start_tag, end_tag, content, nameday_string)
}

fn replace_in_content(
    start_tag: &str,
    end_tag: &str,
    content: &str,
    nameday_string: &str,
) -> String {
    if let (Some(start), Some(end)) = (content.find(start_tag), content.find(end_tag)) {
        let mut new_content = content[..start + start_tag.len()].to_string();
        new_content.push_str(nameday_string);
        new_content.push_str(&content[end..]);
        new_content
    } else {
        content.to_string()
    }
}

async fn update_index_weather() {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=50.0755&longitude=14.4378&current_weather=true&timezone=Europe/Prague";

    let weather_string = match fetch_weather(url).await {
        Ok(temp) => format!("{:.0}°C | Praha", temp),
        Err(_) => "".to_string(),
    };

    if let Ok(content) = fs::read_to_string("index.html") {
        let start_tag = "<!-- WEATHER -->";
        let end_tag = "<!-- /WEATHER -->";

        if let (Some(_start), Some(_end)) = (content.find(start_tag), content.find(end_tag)) {
            let new_content = replace_weather_in_content(&content, &weather_string);

            if content != new_content {
                if let Err(e) = fs::write("index.html", new_content) {
                    error!("Failed to write index.html for weather: {}", e);
                } else {
                    info!("index.html weather updated to: {}", weather_string);
                }
            } else {
                info!(
                    "index.html weather is already up to date: {}",
                    weather_string
                );
            }
        }
    }
}

fn replace_weather_in_content(content: &str, weather_string: &str) -> String {
    let start_tag = "<!-- WEATHER -->";
    let end_tag = "<!-- /WEATHER -->";
    replace_in_content(start_tag, end_tag, content, weather_string)
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
        let weather_string = "23°C | Praha";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(
            new_content,
            "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>"
        );
    }

    #[test]
    fn test_no_weather_replacement_if_same() {
        let content = "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>";
        let weather_string = "23°C | Praha";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(content, new_content);
    }

    #[test]
    fn test_weather_replacement_empty_if_exception() {
        let content = "<html><!-- WEATHER -->23°C | Praha<!-- /WEATHER --></html>";
        let weather_string = "";
        let new_content = replace_weather_in_content(content, weather_string);
        assert_eq!(
            new_content,
            "<html><!-- WEATHER --><!-- /WEATHER --></html>"
        );
    }

    #[test]
    fn test_nameday_replacement_logic() {
        let content = "<html><!-- NAME_DAY -->OLD<!-- /NAME_DAY --></html>";
        let nameday_string = "Svátek má Jaroslava";
        let new_content = replace_nameday_in_content(content, nameday_string);
        assert_eq!(
            new_content,
            "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>"
        );
    }

    #[test]
    fn test_no_nameday_replacement_if_same() {
        let content = "<html><!-- NAME_DAY -->Svátek má Jaroslava<!-- /NAME_DAY --></html>";
        let nameday_string = "Svátek má Jaroslava";
        let new_content = replace_nameday_in_content(content, nameday_string);
        assert_eq!(content, new_content);
    }
}
