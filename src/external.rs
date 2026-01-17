use std::error::Error;

// TODO
pub async fn fetch_weather() -> Result<f64, Box<dyn Error>> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=50.0755&longitude=14.4378&current_weather=true&timezone=Europe/Prague";

    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let temp = resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or("Could not find temperature")?;
    Ok(temp)
}
