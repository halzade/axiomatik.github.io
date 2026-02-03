use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::engine::any::Any;
use surrealdb::opt::Resource;
use surrealdb::Surreal;
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

        let db = self.db.write().await;
        let created: surrealdb::sql::Value = db.create(resource).content(value).await?;

        // SurrealDB returns the record ID under "id"
        match created {
            surrealdb::sql::Value::Object(obj) => {
                if let Some(id_val) = obj.get("id") {
                    Ok(id_val.to_string())
                } else {
                    Err(SurrealError::InvalidStatement(
                        "No id returned from create".into(),
                    ))
                }
            }
            _ => Err(SurrealError::InvalidStatement(
                "Failed to create record - unexpected return value".into(),
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
        let value: surrealdb::sql::Value = db.select(resource).await?;

        match value {
            surrealdb::sql::Value::None | surrealdb::sql::Value::Null => {
                Err(SurrealError::RecordNotFound(table.to_string(), id.to_string()))
            }
            _ => {
                let json = serde_json::to_value(value)
                    .map_err(|e| SurrealError::InvalidStatement(e.to_string()))?;
                let t = serde_json::from_value(json)
                    .map_err(|e| SurrealError::InvalidStatement(e.to_string()))?;
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
