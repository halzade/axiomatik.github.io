use crate::db::database_user::User;
use crate::trust::app::user::user_data::UserFluent;
use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use tracing::error;
use TrustError::Validation;

#[derive(Debug)]
pub struct DatabaseUserVerifier {
    real: User,
    pub expected: UserFluent,
}

impl DatabaseUserVerifier {
    pub fn new(real: User) -> Self {
        Self { real, expected: UserFluent::new() }
    }

    pub fn verify(&self) -> Result<(), TrustError> {
        let mut errors: Vec<String> = Vec::new();
        let expected = self.expected.get_data();

        // username
        if let Some(exp) = expected.username {
            let real = self.real.username.as_str();
            if exp != real {
                errors.push(error("username", exp, real));
            }
        }

        // author_name
        if let Some(exp) = expected.author_name {
            let real = self.real.author_name.as_str();
            if exp != real {
                errors.push(error("author_name", exp, real));
            }
        }

        // role
        if let Some(exp) = expected.role {
            let real = &self.real.role;
            if exp != *real {
                errors.push(error("role", format!("{:?}", exp), &format!("{:?}", real)));
            }
        }

        // needs_password_change
        if let Some(exp) = expected.needs_password_change {
            let real = self.real.needs_password_change;
            if exp != real {
                errors.push(error("needs_password_change", exp.to_string(), &real.to_string()));
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
    pub fn username(&self, username: &str) -> &Self {
        self.expected.username(username);
        self
    }

    pub fn author_name(&self, author_name: &str) -> &Self {
        self.expected.author_name(author_name);
        self
    }

    pub fn role(&self, role: crate::db::database_user::Role) -> &Self {
        self.expected.role(role);
        self
    }

    pub fn needs_password_change(&self, needs: bool) -> &Self {
        self.expected.needs_password_change(needs);
        self
    }
}
