use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct LoginData {
    pub username: Option<String>,
    pub password: Option<String>,
    pub needs_password_change: bool,
}

impl LoginData {
    pub(crate) const fn new() -> Self {
        Self { username: None, password: None, needs_password_change: false }
    }
}

#[derive(Clone, Debug)]
pub struct LoginFluent {
    data: Arc<RwLock<LoginData>>,
}

impl Default for LoginFluent {
    fn default() -> Self {
        Self::new()
    }
}

impl LoginFluent {
    pub fn new() -> Self {
        Self { data: Arc::new(RwLock::new(LoginData::new())) }
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

    /**
     * default false, override if needed
     */
    pub fn needs_password_change(&self, needs: bool) -> &Self {
        let mut guard = self.data.write();
        guard.needs_password_change = needs;
        self
    }

    pub fn get_data(&self) -> LoginData {
        let guard = self.data.read();
        LoginData {
            username: guard.username.clone(),
            password: guard.password.clone(),
            needs_password_change: guard.needs_password_change,
        }
    }
}
