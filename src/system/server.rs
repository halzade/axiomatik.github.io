use crate::system::router;

pub const AUTH_COOKIE: &str = "axiomatik_auth";

struct Server {
    // TODO
}

impl Server {
    // TODO
}

#[derive(Clone, Copy, PartialEq)]
pub enum ApplicationStatus {
    Started,
    Off,
}

impl ApplicationStatus {
    pub fn start() -> Self {
        Self::Started
    }
}

pub async fn start_server() {
    router::start_router(ApplicationStatus::start());
}
