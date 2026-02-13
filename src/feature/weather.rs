use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ExternalError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Temperature data not found")]
    TemperatureNotFound,
}

pub async fn fetch_weather() -> String {
    let weather_r = remote_get_weather().await;
    weather_r.map_or_else(|_| "".to_string(), |weather| format!("{:.0}°C | Praha", weather))
}

async fn remote_get_weather() -> Result<f64, ExternalError> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=50.0755&longitude=14.4378&current_weather=true&timezone=Europe/Prague";

    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let temperature_r = resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or(ExternalError::TemperatureNotFound);
    match temperature_r {
        Ok(temperature) => Ok(temperature),
        Err(e) => {
            error!(error = ?e, "Could not fetch temperature");
            Err(e)
        }
    }
}
