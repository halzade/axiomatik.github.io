pub mod app {
    pub mod test_001_login;
    pub mod test_002_login_username;
    pub mod test_003_login_password;
    pub mod test_004_login_rejection;
    pub mod test_005_change_password;
    pub mod test_006_account_update_author_name;
    pub mod test_007_create_article;
    pub mod test_command_create_editor_user;
    pub mod test_create_article_account_integration;
    pub mod test_create_article_image_upload;
    pub mod test_create_article_is_exclusive_tests;
    pub mod test_create_article_republika_integration;
    pub mod test_create_article_validation;
    pub mod test_create_article_zahranici_integration;
}
pub mod db {
    pub mod test_001_db;
}
pub mod templates {
    pub mod test_article_build_from_template;
    pub mod test_category_finance_build_from_template;
    pub mod test_category_republika_build_from_template;
    pub mod test_category_technologie_build_from_template;
    pub mod test_category_veda_build_from_template;
    pub mod test_category_zahranici_build_from_template;
    pub mod test_index_build_from_template;
    pub mod test_news_build_from_template;
}
pub mod web {
    pub mod test_001_fallback_404;
    pub mod test_002_serve_static_content;
}
