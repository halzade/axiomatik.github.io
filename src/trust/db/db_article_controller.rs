use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct ArticleDatabaseController {}

impl ArticleDatabaseController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(self) -> Result<(), TrustError> {
        // TODO response

        Ok(())
    }
}
