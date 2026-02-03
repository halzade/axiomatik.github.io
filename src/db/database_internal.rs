use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::engine::any::Any;
use surrealdb::types::{SurrealValue, Value};
use surrealdb::{opt::Resource, Surreal};
use thiserror::Error;
use tokio::sync::RwLock;
use SurrealError::{InvalidStatement, RecordNotFound};

const DATABASE: &str = "file://axiomatik.db";
const DATABASE_TEST: &str = "memory://axiomatik.test";

#[derive(Debug, Error)]
pub enum SurrealError {
    #[error("surreal db error {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("serde json error {0}")]
    Serde(#[from] serde_json::Error),

    #[error("invalid statement")]
    InvalidStatement,

    #[error("record not found {0}, id {1}")]
    RecordNotFound(String, String),
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
        T: Serialize + SurrealValue + Clone,
    {
        let resource = match id {
            Some(id) => Resource::from((table, id)),
            None => Resource::from(table),
        };

        let db = self.db.write().await;
        // Pass owned value so it implements the expected SurrealValue (not &T)
        let created: Value = db.create(resource).content(value.clone()).await?;

        match created {
            Value::Object(obj) => {
                if let Some(id_val) = obj.get("id") {
                    // Try to extract a string id; if not a string, fall back to JSON string
                    let id_json = serde_json::to_value(id_val)?;
                    let id_str = match id_json {
                        serde_json::Value::String(s) => s,
                        other => other.to_string(),
                    };
                    Ok(id_str)
                } else {
                    Err(InvalidStatement)
                }
            }
            _ => Err(InvalidStatement),
        }
    }

    /// Read a record by table + id and deserialize to struct T
    pub async fn read_struct<T>(&self, table: &str, id: &str) -> Result<T, SurrealError>
    where
        T: DeserializeOwned,
    {
        let resource = Resource::from((table, id));
        let db = self.db.read().await;
        let value: Value = db.select(resource).await?;

        match value {
            Value::None | Value::Null => Err(RecordNotFound(table.to_string(), id.to_string())),
            other => {
                let json = serde_json::to_value(other)?;
                let t: T = serde_json::from_value(json)?;
                Ok(t)
            }
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
