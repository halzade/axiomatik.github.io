#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]
#![forbid(clippy::todo)]
#![forbid(clippy::unimplemented)]

pub mod db {
    pub mod database;
    pub mod database_article;
    pub mod database_internal;
    pub mod database_system;
    pub mod database_user;
}
pub mod feature {
    pub mod name_days;
    pub mod name_days_library;
    pub mod weather;
}
pub mod form {
    pub mod form_account;
    pub mod form_category;
    pub mod form_change_password;
    pub mod form_index;
    pub mod form_login;
    pub mod form_new_article;
    pub mod form_new_article_data;
    pub mod form_search;
}
pub mod library;
pub mod logger;
pub mod processor;
pub mod system {
    pub mod system_data;
    pub mod configuration;
    pub mod server;
    pub mod commands;
}
pub mod utils;
pub mod validation {
    pub mod validate_media;
    pub mod validate_text;
}
pub mod test_framework {
    pub mod article_builder;
    pub mod script_base;
    pub mod script_base_data;
}
