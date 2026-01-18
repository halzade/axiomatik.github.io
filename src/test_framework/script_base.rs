use std::convert::Into;
use axum::Router;
use http::{header, Request, Response};

use std::string::ToString;
use tower::ServiceExt;

use crate::database::Role::Editor;
use crate::database::User;
use crate::{database, logger, server};
use std::sync::OnceLock;
use tokio::sync::OnceCell;
use tracing::{info};

static SETUP_ONCE: OnceCell<()> = OnceCell::const_new();

static APP_ROUTER: OnceLock<Router> = OnceLock::new();
static ORIGINAL_INDEX: OnceCell<String> = OnceCell::const_new();

const PASSWORD: &str = "password123";

struct Cleanup;

impl Drop for Cleanup {
    /*
     * Method drop() is called with destructor
     * Since Cleanup is static struct, it will be called when all tests finished
     */
    fn drop(&mut self) {
        after_tests_clean_up();
    }
}

pub async fn setup_before_tests_once() {
    info!("setup_before_tests()");
    SETUP_ONCE
        .get_or_init(|| async {
            info!("setup_before_tests() Executes");

            info!("register clean up");
            let _cleanup = Cleanup;

            // as if in main.rs
            server::start().await;
            logger::config();

            info!("init database");
            database::initialize_in_memory_database().await;

            info!("init server");
            // let rt = tokio::runtime::Runtime::new().unwrap();
            // let router = rt.block_on(server::router());

            //
            // TODO is app was already running in devel
            let router = server::router().await;
            // let config = configuration::get_config().expect("Failed to read configuration for tests.");
            // let addr = format!("{}:{}", config.host, config.port);
            // info!("listening on {}", addr);
            // let listener = TcpListener::bind(&addr)
            //   .await
            //    .expect(&format!("Failed to bind to {}", addr));
            /*
             * Start Application
             */
            // let serve_r = axum::serve(listener, router).await;
            // match serve_r {
            //     Ok(serve) => {
            //
            //     }
            //     Err(e) => {
            //         error!("axum server exited: {:?}", e);
            //     }
            // }
            //
            APP_ROUTER.set(router).expect("error");

            let original_index = std::fs::read_to_string("index.html").unwrap();
            ORIGINAL_INDEX.set(original_index).unwrap();
        })
        .await;
}

fn after_tests_clean_up() {
    info!("All tests finished!");
    if let Some(content) = ORIGINAL_INDEX.get() {
        info!("rewrite index from original");
        std::fs::write("index.html", content).unwrap();
    }
}

pub async fn one_shot(request: Request<reqwest::Body>) -> Response<axum::body::Body> {
    let router = APP_ROUTER.get().unwrap().clone();
    router.oneshot(request).await.unwrap()
}

pub fn serialize(params: &[(&str, &str)]) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    serializer.extend_pairs(params);
    serializer.finish()
}

pub fn original_index() -> String {
    ORIGINAL_INDEX.get().unwrap().to_string()
}

pub async fn setup_user_and_login(name: &str) -> String {
    // create user in DB
    database::create_user(new_user(name)).await.unwrap();

    // login user
    let login_resp = one_shot(
        Request::builder()
            .method("POST")
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(reqwest::Body::from(format!(
                "username={}&password={}",
                name, PASSWORD
            )))
            .unwrap(),
    )
    .await;

    let cookie = login_resp
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    cookie
}

fn new_user(name: &str) -> User {
    let password_hash = bcrypt::hash(PASSWORD, bcrypt::DEFAULT_COST).unwrap();
    User {
        username: name.into(),
        author_name: name.into(),
        password_hash,
        needs_password_change: false,
        role: Editor,
    }
}
