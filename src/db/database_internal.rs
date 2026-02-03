use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_value, to_value};
use surrealdb::engine::any::Any;
use surrealdb::types::Value;
use surrealdb::{opt::Resource, Surreal};
use thiserror::Error;
use tokio::sync::RwLock;
use SurrealError::{InvalidStatement, RecordNotFound};

const DATABASE: &str = "rocksdb://axiomatik.db";
const DATABASE_TEST: &str = "mem://";

#[derive(Debug, Error)]
pub enum SurrealError {
    #[error("surreal db error {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("serde json error {0}")]
    Serde(#[from] serde_json::Error),

    #[error("invalid statement")]
    InvalidStatement,

    #[error("record not found in table {0} by key {1}")]
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

    pub async fn create_struct<NewT>(&self, table: &str, value: &NewT) -> Result<(), SurrealError>
    where
        NewT: Serialize,
    {
        let db = self.db.write().await;

        // Serialize struct to JSON first
        let json = to_value(value)?;

        let _: Option<Value> = db.create(table).content(json).await?;

        Ok(())
    }

    pub async fn update_struct<T>(&self, table: &str, value: &T) -> Result<(), SurrealError>
    where
        T: Serialize + Clone,
    {
        // Extract id from struct via JSON
        let json = to_value(value)?;
        let id = json.get("id").ok_or(InvalidStatement)?;

        let id_str = match id {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };

        let resource = Resource::from((table, id_str.as_str()));
        let db = self.db.write().await;

        let json = to_value(value)?;

        let updated: Value = db.update(resource).content(json).await?;

        match updated {
            Value::Object(_) => Ok(()),
            _ => Err(InvalidStatement),
        }
    }

    pub async fn delete_struct<T>(
        &self,
        table: &str,
        struct_to_delete: T,
    ) -> Result<(), SurrealError>
    where
        T: Serialize,
    {
        // Extract `id` from struct via JSON
        let json = to_value(&struct_to_delete)?;
        let id = json.get("id").ok_or(InvalidStatement)?;

        let id_str = match id {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };

        let resource = Resource::from((table, id_str.as_str()));
        let db = self.db.write().await;

        let _idempotent: Value = db.delete(resource).await?;

        Ok(())
    }

    pub async fn read_by_key<T>(&self, table: &str, key: &str) -> Result<T, SurrealError>
    where
        T: DeserializeOwned,
    {
        let resource = Resource::from((table, key));
        let db = self.db.read().await;
        let value: Value = db.select(resource).await?;

        match value {
            Value::None | Value::Null => Err(RecordNotFound(table.to_string(), key.to_string())),
            other => {
                let json = to_value(other)?;
                let t: T = from_value(json)?;
                Ok(t)
            }
        }
    }
}

pub async fn init_db() -> DatabaseSurreal {
    DatabaseSurreal::new(DATABASE)
        .await
        .expect("Failed to initialize database")
}

pub async fn init_db_test() -> DatabaseSurreal {
    DatabaseSurreal::new(DATABASE_TEST)
        .await
        .expect("Failed to initialize test database")
}
