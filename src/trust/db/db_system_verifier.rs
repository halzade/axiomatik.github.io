use crate::db::database_system::ArticleStatus;
use crate::trust::app::system::system_data::SystemFluent;
use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use tracing::error;
use TrustError::Validation;

#[derive(Debug)]
pub struct DatabaseSystemVerifier {
    real: ArticleStatus,
    pub expected: SystemFluent,
}

impl DatabaseSystemVerifier {
    pub fn new(real: ArticleStatus) -> Self {
        Self { real, expected: SystemFluent::new() }
    }

    pub fn verify(&self) -> Result<(), TrustError> {
        let mut errors: Vec<String> = Vec::new();
        let expected = self.expected.get_data();

        // article_status
        if let Some(exp) = expected.article_status {
            let real = self.real;
            if exp != real {
                errors.push(error("article_status", format!("{:?}", exp), &format!("{:?}", real)));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            for e in &errors {
                error!("{}", e);
            }
            Err(Validation(format!("{} incorrect", errors.len())))
        }
    }

    /*
     * fluent interface methods
     */
    pub fn article_status(&self, status: ArticleStatus) -> &Self {
        self.expected.article_status(status);
        self
    }
}
