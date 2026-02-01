use crate::application::article::article::ArticleError;
use crate::application::form::form_article_data_parser::ArticleCreateError;
use crate::db::database_user;
use crate::system::server::AUTH_COOKIE;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormArticleCreateError {
    #[error("article error")]
    FormArticleError(#[from] ArticleError),

    #[error("article create error")]
    FormArticleCreateError(#[from] ArticleCreateError),
}

#[derive(Template)]
#[template(path = "application/article/form.html")]
pub struct FormTemplate {
    pub author_name: String,
}

pub async fn show_article_create_form(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        let user_o = database_user::get_user(cookie.value()).await;
        match user_o {
            None => {}
            Some(user) => {
                return Html(
                    FormTemplate {
                        author_name: user.author_name,
                    }
                    .render()
                    .unwrap(),
                )
                .into_response();
            }
        }
    }
    Redirect::to("/login").into_response()
}

impl IntoResponse for FormArticleCreateError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
