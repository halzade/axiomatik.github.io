use crate::application::article::form_article_data_parser;
use crate::application::article::form_article_data_parser::ArticleCreateError;
use crate::db::database_user;
use crate::system::server::AUTH_COOKIE;
use crate::web::article::article;
use crate::web::article::article::ArticleError;
use askama::Template;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormError {
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

pub async fn show_form(jar: CookieJar) -> Response {
    if let Some(cookie) = jar.get(AUTH_COOKIE) {
        // TODO but his is already handled by middleware layer
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

impl IntoResponse for FormError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

pub async fn create_article(multipart: Multipart) -> Result<impl IntoResponse, FormError> {
    // TODO article already exists
    // TODO doubled request on create button

    /*
     * Read request data
     */
    let article_data = form_article_data_parser::article_data(multipart).await?;

    /*
     * Create Article, process the data
     */
    let article_url = article::process_article_create(article_data).await?;

    Ok(Redirect::to(&article_url).into_response())
}
