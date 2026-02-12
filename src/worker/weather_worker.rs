use std::time::Duration;
use thiserror::Error;
use tokio::time::interval;
use tracing::{info, trace};

#[derive(Debug, Error)]
pub enum WeatherWorkerError {
    #[error("weather error")]
    Weather,
}

pub fn start_weather_worker() -> Result<(), WeatherWorkerError> {
    info!("start weather worker");
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_mins(30));
        loop {
            interval.tick().await;
            trace!("fetch weather");
        }
    });

    Ok(())
}
