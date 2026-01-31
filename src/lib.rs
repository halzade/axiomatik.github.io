/*
 * Lazybones!
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
    pub mod form_account;
    pub mod form_article_create;
    pub mod form_article_data_parser;
    pub mod form_change_password;
    pub mod form_login;
}
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
pub mod library;
pub mod logger;
pub mod processor {
    pub mod process_audio;
    pub mod process_images;
    pub mod process_text;
    pub mod process_video;
    pub mod processor;
}
pub mod system {
    pub mod commands;
    pub mod configuration;
    pub mod content;
    pub mod heartbeat;
    pub mod router;
    pub mod server;
    pub mod system_data;
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
pub mod web {
    pub mod base;
    pub mod article {
        pub mod article;
    }
    pub mod index {
        pub mod index;
    }
    pub mod finance {
        pub mod finance;
    }
    pub mod republika {
        pub mod republika;
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
    pub mod news {
        pub mod news;
    }
    pub mod search {
        pub mod search;
    }
}
