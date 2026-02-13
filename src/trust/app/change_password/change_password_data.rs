use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct ChangePasswordData {
    pub new_password: Option<String>,
}

impl ChangePasswordData {
    pub(crate) fn new() -> ChangePasswordData {
        ChangePasswordData::default()
    }
}

#[derive(Clone, Debug)]
pub struct ChangePasswordFluent {
    pub(crate) data: Arc<RwLock<ChangePasswordData>>,
}

impl Default for ChangePasswordFluent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangePasswordFluent {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(ChangePasswordData::new())),
        }
    }

    pub fn new_password(&self, password: &str) -> &Self {
        let mut guard = self.data.write();
        guard.new_password = Some(password.to_string());
        self
    }

    pub fn get_data(&self) -> ChangePasswordData {
        let guard = self.data.read();
        ChangePasswordData {
            new_password: guard.new_password.clone(),
        }
    }
}
