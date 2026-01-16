use config::{Config, ConfigError, File};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `dev` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| {
            info!("APP_ENVIRONMENT not set, defaulting to 'dev'");
            "dev".into()
        })
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let settings = Config::builder()
        .add_source(File::from(
            configuration_directory.join(format!("{}.toml", environment.as_str())),
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Dev,
    Prod,
}

// runs onf different port
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{} is not a supported environment. Use either `dev` or `prod`.",
                other
            )),
        }
    }
}
