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
    pub mod article {
        pub mod form_article_create;
        pub mod form_article_data_parser;
    }
    pub mod account {
        pub mod form_account;
    }
    pub mod change_password {
        pub mod form_change_password;
    }
    pub mod login {
        pub mod form_login;
    }
}
pub mod db {
    pub mod database;
    pub mod database_article;
    pub mod database_internal;
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
pub mod library;
pub mod logger;
pub mod system {
    pub mod commands;
    pub mod configuration;
    pub mod content;
    pub mod heartbeat;
    pub mod router;
    pub mod server;
    pub mod system_data;
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
