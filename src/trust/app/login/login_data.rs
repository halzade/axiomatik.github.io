use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct LoginData {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl LoginData {
    pub(crate) fn new() -> LoginData {
        LoginData { username: None, password: None }
    }
}

#[derive(Clone, Debug)]
pub struct LoginFluent {
    data: Arc<RwLock<LoginData>>,
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

    pub fn get_data(&self) -> LoginData {
        let guard = self.data.read();
        LoginData { username: guard.username.clone(), password: guard.password.clone() }
    }
}
