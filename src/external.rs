async fn fetch_weather(url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let temp = resp["current_weather"]["temperature"]
        .as_f64()
        .ok_or("Could not find temperature")?;
    Ok(temp)
}