use crate::system::server::TheState;
use std::time::Duration;
use thiserror::Error;
use tokio::time::interval;
use tracing::{info, trace};

#[derive(Debug, Error)]
pub enum WeatherWorkerError {
    #[error("weather error")]
    Weather,
}

pub fn start_weather_worker(state: TheState) -> Result<(), WeatherWorkerError> {
    info!("start weather worker");
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_mins(30));

        let state_c = state.clone();

        loop {
            tokio::spawn(async move {
                trace!("fetch weather");

                let changed = state_c.ds.update_weather().await;

                if changed {
                    // weather changed

                    // TODO invalidate everything
                }
            });
            interval.tick().await;
        }
    });

    Ok(())
}
