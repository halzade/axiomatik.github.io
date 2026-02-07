use crate::db::database_user::User;
use crate::trust::app::user::user_data::UserFluent;
use crate::trust::data::utils::error;
use crate::trust::me::TrustError;
use TrustError::Validation;

#[derive(Debug)]
pub struct DatabaseUserVerifier {
    real: User,
    pub expected: UserFluent,
}

impl DatabaseUserVerifier {
    pub fn new(real: User) -> Self {
        Self {
            real,
            expected: UserFluent::new(),
        }
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

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Validation(errors.join("\n")))
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
}
