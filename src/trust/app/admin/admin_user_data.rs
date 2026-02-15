use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct AdminUserData {
    pub username: Option<String>,
}

impl AdminUserData {
    pub const fn new() -> Self {
        Self { username: None }
    }
}

#[derive(Clone, Debug)]
pub struct AdminUserFluent {
    data: Arc<RwLock<AdminUserData>>,
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

    pub fn get_data(&self) -> AdminUserData {
        let guard = self.data.read();
        AdminUserData {
            username: guard.username.clone(),
        }
    }
}
