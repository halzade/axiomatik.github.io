use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct AdminUserData {
    pub username: Option<String>,
    pub password: Option<String>,
    pub author_name: Option<String>,
}

impl AdminUserData {
    pub const fn new() -> Self {
        Self {
            username: None,
            password: None,
            author_name: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AdminUserFluent {
    pub data: Arc<RwLock<AdminUserData>>,
}

impl Default for AdminUserFluent {
    fn default() -> Self {
        Self::new()
    }
}

impl AdminUserFluent {
    pub fn new() -> Self {
        Self { data: Arc::new(RwLock::new(AdminUserData::new())) }
    }

    pub fn username(&self, username: &str) -> &Self {
        let mut guard = self.data.write();
        guard.username = Some(username.to_string());
        self
    }

    pub fn password(&self, password: &str) -> &Self {
        let mut guard = self.data.write();
        guard.password = Some(password.to_string());
        self
    }

    pub fn author_name(&self, author_name: &str) -> &Self {
        let mut guard = self.data.write();
        guard.author_name = Some(author_name.to_string());
        self
    }

    pub fn get_data(&self) -> AdminUserData {
        let guard = self.data.read();
        AdminUserData {
            username: guard.username.clone(),
            password: guard.password.clone(),
            author_name: guard.author_name.clone(),
        }
    }

    pub fn reset(&self) {
        *self.data.write() = AdminUserData::new();
    }
}
