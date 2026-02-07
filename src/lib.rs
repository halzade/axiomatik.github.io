/*
 * Lazybones!
 * cargo clippy
 */
#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]
#![forbid(clippy::todo)]
#![forbid(clippy::unimplemented)]
/*
 * Implementation
 */
pub mod application {
    pub mod account {
        pub mod form_account;
    }
    pub mod article {
        pub mod article;
    }
    pub mod change_password {
        pub mod form_change_password;
    }
    pub mod finance {
        pub mod finance;
    }
    pub mod form {
        pub mod form_article_create;
        pub mod form_article_data_parser;
    }
    pub mod index {
        pub mod index;
    }
    pub mod login {
        pub mod form_login;
    }
    pub mod news {
        pub mod news;
    }
    pub mod republika {
        pub mod republika;
    }
    pub mod search {
        pub mod search;
    }
    pub mod technologie {
        pub mod technologie;
    }
    pub mod veda {
        pub mod veda;
    }
    pub mod zahranici {
        pub mod zahranici;
    }
}
pub mod db {
    pub mod database;
    pub mod database_article;
    pub mod database_article_data;
    pub mod database_system;
    pub mod database_user;
}
pub mod data {
    pub mod audio_extractor;
    pub mod audio_processor;
    pub mod audio_validator;
    pub mod image_extractor;
    pub mod image_processor;
    pub mod image_validator;
    pub mod library;
    pub mod processor;
    pub mod text_extractor;
    pub mod text_processor;
    pub mod text_validator;
    pub mod video_extractor;
    pub mod video_processor;
    pub mod video_validator;
}
pub mod feature {
    pub mod name_days;
    pub mod name_days_library;
    pub mod weather;
}
pub mod system {
    pub mod authentication;
    pub mod commands;
    pub mod configuration;
    pub mod data_system;
    pub mod data_updates;
    pub mod heartbeat;
    pub mod logger;
    pub mod router_app;
    pub mod router_web;
    pub mod server;
}
/*
 * Test Framework
 */
pub mod trust {
    pub mod app {
        pub mod article {
            pub mod create_article_controller;
            pub mod create_article_data;
            pub mod create_article_easy_builder;
            pub mod create_article_request_builder;
        }
        pub mod user {
            pub mod user_data;
        }
        pub mod account {
            pub mod account_controller;
            pub mod account_data;
        }
        pub mod change_password {
            pub mod change_password_controller;
            pub mod change_password_data;
        }
        pub mod login {
            pub mod login_controller;
            pub mod login_data;
        }
    }
    pub mod app_controller;
    pub mod db {
        pub mod db_article_controller;
        pub mod db_article_verifier;
        pub mod db_system_controller;
        pub mod db_system_verifier;
        pub mod db_user_controller;
        pub mod db_user_verifier;
    }
    pub mod data {
        pub mod media_data;
        pub mod response_verifier;
        pub mod utils;
    }
    pub mod me;
    pub mod web {
        pub mod web_controller;
    }
}
