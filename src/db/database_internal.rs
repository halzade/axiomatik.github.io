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

    pub async fn create_struct<NewT>(&self, table: &str, value: &NewT) -> Result<(), SurrealError>
    where
        NewT: Serialize + SurrealValue + Clone,
    {
        let db = self.db.write().await;

        let created_o: Option<Value> = db.create(table).content(value.clone()).await?;

        match created_o {
            None => Ok(()),
            Some(_) => Err(InvalidStatement),
        }
    }

    pub async fn update_struct<T>(&self, table: &str, value: &T) -> Result<(), SurrealError>
    where
        T: Serialize + SurrealValue + Clone,
    {
        // Extract id from struct via JSON
        let json = serde_json::to_value(value)?;
        let id = json.get("id").ok_or(InvalidStatement)?;

        let id_str = match id {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };

        let resource = Resource::from((table, id_str.as_str()));
        let db = self.db.write().await;

        let updated: Value = db.update(resource).content(value.clone()).await?;

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
        let json = serde_json::to_value(&struct_to_delete)?;
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

    /// Read a record by table + id and deserialize to struct T
    pub async fn read_by_id<T>(&self, table: &str, id: &str) -> Result<T, SurrealError>
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
