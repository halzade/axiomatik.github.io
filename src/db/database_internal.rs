use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use surrealdb::engine::any::Any;
use surrealdb::{Surreal, opt::Resource};
use surrealdb::types::Value;
use tokio::sync::RwLock;

const DATABASE: &str = "file://axiomatik.db";
const DATABASE_TEST: &str = "memory://axiomatik.test";

#[derive(Debug)]
pub enum SurrealError {
    InvalidStatement(String),
    RecordNotFound(String, String),
    Surreal(surrealdb::Error),
}

impl From<surrealdb::Error> for SurrealError {
    fn from(err: surrealdb::Error) -> Self {
        SurrealError::Surreal(err)
    }
}

pub struct DatabaseSurreal {
    // Any is for {Local, Remote}
    pub db: RwLock<Surreal<Any>>,
}

impl DatabaseSurreal {
    pub async fn new(path: &str) -> surrealdb::Result<Self> {
        // infer db engine
        let surreal = surrealdb::engine::any::connect(path).await?;
        surreal.use_ns("axiomatik").use_db("axiomatik").await?;

        Ok(Self {
            db: RwLock::new(surreal),
        })
    }

    /// Write a struct into a table, optionally specifying an ID.
    /// Returns the created record's ID as String
    pub async fn write_struct<T>(
        &self,
        table: &str,
        id: Option<&str>,
        value: &T,
    ) -> Result<String, SurrealError>
    where
        T: Serialize,
    {
        let resource = match id {
            Some(id) => Resource::from((table, id)),
            None => Resource::from(table),
        };

        let mut db = self.db.write().await;
        let created: Value = db.create(resource).content(value).await?;

        match created {
            Value::Object(obj) => {
                if let Some(id_val) = obj.get("id").and_then(|v| v.as_str()) {
                    Ok(id_val.to_string())
                } else {
                    Err(SurrealError::InvalidStatement(
                        "No id returned from create".into(),
                    ))
                }
            }
            Some(_) => Err(SurrealError::InvalidStatement(
                "Failed to create record - unexpected return value".into(),
            )),
            None => Err(SurrealError::InvalidStatement(
                "Failed to create record - no value returned".into(),
            )),
        }
    }

    /// Read a record by table + id and deserialize to struct T
    pub async fn read_struct<T>(&self, table: &str, id: &str) -> Result<T, SurrealError>
    where
        T: DeserializeOwned,
    {
        let resource = Resource::from((table, id));
        let db = self.db.read().await;
        let value: Option<Value> = db.select(resource).await?;

        match value {
            Some(v) => {
                let json = serde_json::to_value(v)
                    .map_err(|e| SurrealError::InvalidStatement(e.to_string()))?;
                let t = serde_json::from_value(json)
                    .map_err(|e| SurrealError::InvalidStatement(e.to_string()))?;
                Ok(t)
            }
            None => Err(SurrealError::RecordNotFound(format!(
                "{}:{}",
                table, id
            ))),
        }
    }
}

pub async fn init_db() -> DatabaseSurreal {
    DatabaseSurreal::new(DATABASE)
        .await
        .expect("Failed to initialize SurrealKV database")
}

pub async fn init_db_test() -> DatabaseSurreal {
    DatabaseSurreal::new(DATABASE_TEST)
        .await
        .expect("Failed to initialize SurrealKV test database")
}
