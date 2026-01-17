use crate::database;
use anyhow::Error;
use serde_json::Value;
use tracing::info;

pub async fn print_from_db(query: &str) -> Result<(), Error> {
    let mut response = database::query(fix_query(query)).await;

    // Check for errors in the response and keep the response
    response = response.check()?;

    // Print all results from the first statement as JSON
    let results: Vec<Value> = response.take(0)?;
    for result in results {
        info!("{}", serde_json::to_string_pretty(&result)?);
    }
    Ok(())
}

fn fix_query(query: &str) -> String {
    let mut query = query.trim().to_string();
    if !query.ends_with(';') {
        query.push(';');
    }
    query
}
