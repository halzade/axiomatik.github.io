use crate::db::database_system::DatabaseSystem;
use crate::trust::me::TrustError;
use std::sync::Arc;

#[derive(Debug)]
pub struct DatabaseSystemController {
    dbs: Arc<DatabaseSystem>,
}

impl DatabaseSystemController {
    pub fn new(dbs: Arc<DatabaseSystem>) -> Self {
        Self { dbs }
    }

    pub fn execute(self) -> Result<(), TrustError> {
        Ok(())
    }
}
