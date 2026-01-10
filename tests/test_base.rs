use axiomatik_web::{app, db};
use std::sync::Arc;
use url::form_urlencoded;

pub fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

pub async fn setup_app() -> (axum::Router, Arc<db::Database>) {
    let db = Arc::new(db::init_mem_db().await);
    (app(db.clone()), db)
}
