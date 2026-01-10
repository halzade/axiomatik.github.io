use crate::db::Database;
use serde_json::Value;

pub async fn print_from_db(db: &Database, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut query = query.trim().to_string();
    if !query.ends_with(';') {
        query.push(';');
    }

    let mut response = db.read().await.query(&query).await?;
    
    // Check for errors in the response and keep the response
    response = response.check()?;

    // Print all results from the first statement as JSON
    let results: Vec<Value> = response.take(0)?;
    for result in results {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}
