use crate::database;
// TODO I don't think these Serde are necessary
use serde_json::Value;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum DatabaseToolsError {
    #[error("Database error: {0}")]
    Database(#[from] database::DatabaseError),
    #[error("SurrealDB error: {0}")]
    Surreal(#[from] surrealdb::Error),
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub async fn print_from_db(query: &str) -> Result<(), DatabaseToolsError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_query() {
        assert_eq!(fix_query("SELECT * FROM table"), "SELECT * FROM table;");
        assert_eq!(fix_query("SELECT * FROM table;"), "SELECT * FROM table;");
        assert_eq!(fix_query("  SELECT * FROM table  "), "SELECT * FROM table;");
    }
}
