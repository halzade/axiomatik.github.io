use crate::trust::me::TrustError;

#[derive(Debug)]
pub struct DatabaseArticleController {}

impl DatabaseArticleController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(self) -> Result<(), TrustError> {
        // TODO response

        Ok(())
    }
}
