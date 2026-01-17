use anyhow::Error;
use tracing::error;

pub async fn fetch_weather() -> String {
    let weather_r = remote_get_weather().await;
    match weather_r {
        Ok(weather) => {
            format!("{:.0}°C | Praha", weather)
        }
        Err(_) => "".to_string(),
    }
}

async fn remote_get_weather() -> Result<f64, Error> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=50.0755&longitude=14.4378&current_weather=true&timezone=Europe/Prague";

    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let temperature_r = resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or("Could not find temperature");
    match temperature_r {
        Ok(temperature) => Ok(temperature),
        Err(e) => {
            error!(error = ?e, "Could not fetch temperature");
            Err(anyhow::anyhow!("anyhow, temperature"))
        }
    }
}
