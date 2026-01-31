use crate::application::article::form_article_data_parser;
use crate::application::article::form_article_data_parser::ArticleError;
use crate::db::database_user;
use crate::processor::processor;
use crate::system::server::AUTH_COOKIE;
use askama::Template;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;

#[derive(Template)]
#[template(path = "article/form.html")]
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

pub async fn create_article(
    jar: CookieJar,
    multipart: Multipart,
) -> Result<Response, ArticleError> {
    let created_by = if let Some(cookie) = jar.get(AUTH_COOKIE) {
        cookie.value().to_string()
    } else {
        return Ok(Redirect::to("/login").into_response());
    };

    // TODO article already exists
    // TODO double click on create button

    /*
     * Read request data
     */
    let article_data = form_article_data_parser::article_data(multipart).await?;

    /*
     * Create Article, process the data
     */
    let article_url_r = processor::process_articles_create(article_data);

    match article_url_r {
        Ok(article_url) => Ok(Redirect::to(&article_url).into_response()),
        Err(e) => Err(e),
    }
}
