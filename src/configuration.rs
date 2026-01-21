use config::{Config, ConfigError, File};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<ApplicationSettings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let settings = Config::builder()
        .add_source(File::from(
            /*
             * Configuration file
             */
            configuration_directory.join(format!("{}.toml", my_env_is())),
        ))
        .build()?;

    settings.get::<ApplicationSettings>("application")
}

fn my_env_is() -> String {
    std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| {
        info!("APP_ENVIRONMENT not set, defaulting to 'dev'");
        "dev".into()
    })
}
