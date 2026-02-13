use crate::db::database_user::Role;
use parking_lot::RwLock;
use std::sync::Arc;

/**
 * user fluent interface
 */
#[derive(Debug)]
pub struct UserData {
    pub username: Option<String>,
    pub author_name: Option<String>,
    pub role: Option<Role>,
    pub needs_password_change: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct UserFluent {
    data: Arc<RwLock<UserData>>,
}

impl Default for UserFluent {
    fn default() -> Self {
        Self::new()
    }
}

impl UserFluent {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(UserData {
                username: None,
                author_name: None,
                role: None,
                needs_password_change: None,
            })),
        }
    }

    pub fn username(&self, username: &str) -> &Self {
        let mut guard = self.data.write();
        guard.username = Some(username.to_string());
        self
    }

    pub fn author_name(&self, author_name: &str) -> &Self {
        let mut guard = self.data.write();
        guard.author_name = Some(author_name.to_string());
        self
    }

    pub fn role(&self, role: Role) -> &Self {
        let mut guard = self.data.write();
        guard.role = Some(role);
        self
    }

    pub fn needs_password_change(&self, needs: bool) -> &Self {
        let mut guard = self.data.write();
        guard.needs_password_change = Some(needs);
        self
    }

    // Safe read access (no poison, no unwrap)
    pub fn get_data(&self) -> UserData {
        let guard = self.data.read();
        UserData {
            username: guard.username.clone(),
            author_name: guard.author_name.clone(),
            role: guard.role.clone(),
            needs_password_change: guard.needs_password_change,
        }
    }
}
