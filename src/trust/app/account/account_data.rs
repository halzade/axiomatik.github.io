use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct AccountUpdateAuthorData {
    pub author_name: Option<String>,
}

impl AccountUpdateAuthorData {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug)]
pub struct AccountUpdateAuthorFluent {
    pub(crate) data: Arc<RwLock<AccountUpdateAuthorData>>,
}

impl Default for AccountUpdateAuthorFluent {
    fn default() -> Self {
        Self::new()
    }
}

impl AccountUpdateAuthorFluent {
    pub fn new() -> Self {
        Self { data: Arc::new(RwLock::new(AccountUpdateAuthorData::new())) }
    }

    pub fn author_name(&self, author_name: &str) -> &Self {
        let mut guard = self.data.write();
        guard.author_name = Some(author_name.to_string());
        self
    }

    pub fn get_data(&self) -> AccountUpdateAuthorData {
        let guard = self.data.read();
        AccountUpdateAuthorData { author_name: guard.author_name.clone() }
    }
}
