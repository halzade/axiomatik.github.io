use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env::VarError;
use thiserror::Error;
use tracing::{error, warn};
use AppEnvironment::{Dev, Prod, Test};
use VarError::{NotPresent, NotUnicode};

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("failed to determine current working directory")]
    CurrentDirectoryError(#[from] std::io::Error),

    #[error("failed to build configuration")]
    ConfigBuildError(#[from] ConfigError),
}

#[derive(Debug)]
pub enum AppEnvironment {
    Dev,
    Test,
    Prod,
}

impl AppEnvironment {
    pub fn text(self) -> &'static str {
        match self {
            Dev => "dev",
            Test => "staging",
            Prod => "prod",
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: PortSettings,
}

#[derive(Deserialize, Clone)]
pub struct PortSettings {
    pub app: u16,
    pub web: u16,
}

/*
 * Read configuration from ~/configuration/abc.toml
 */
pub fn get_config() -> Result<ApplicationSettings, ConfigurationError> {
    let base_path = std::env::current_dir()?;
    let config_path = base_path
        .join("configuration")
        .join(format!("{}.toml", my_env_is().text()));

    let settings = Config::builder()
        .add_source(File::from(config_path))
        .build()?
        .get::<ApplicationSettings>("application")?;

    Ok(settings)
}

fn my_env_is() -> AppEnvironment {
    match std::env::var("APP_ENVIRONMENT").as_deref() {
        Ok("prod") => Prod,
        Ok("test") => Test,
        Ok("dev") => Dev,
        Ok(other) => {
            warn!(
                "Invalid APP_ENVIRONMENT value '{}', defaulting to 'dev'",
                other
            );
            Dev
        }
        Err(NotPresent) => {
            warn!("APP_ENVIRONMENT not set, defaulting to 'dev'");
            Dev
        }
        Err(NotUnicode(_)) => {
            error!("APP_ENVIRONMENT contains invalid Unicode, defaulting to 'dev'");
            Dev
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::system::configuration;

    #[test]
    fn test_get_config() {
        assert!(configuration::get_config().is_ok());
    }
}
