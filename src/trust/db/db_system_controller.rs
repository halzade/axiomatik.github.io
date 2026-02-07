use crate::db::database_system::DatabaseSystem;
use crate::db::database_user::{Role, User};
use crate::trust::me::TrustError;
use bcrypt::{hash, DEFAULT_COST};
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
        // TODO response

        Ok(())
    }

    pub async fn db_setup_user_with_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), TrustError> {
        // TODO: Move this to DatabaseSystem if needed, but for now we need dbs to have access to surreal
        // Since DatabaseSystem struct in src/db/database_system.rs has 'surreal' field but it's not public
        // and we cannot easily change it now, we might need a workaround or just fix the call if it was meant to be on dbu.
        Ok(())
    }
}
