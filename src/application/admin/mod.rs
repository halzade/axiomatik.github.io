use crate::db::database_article_data::ShortArticleData;
use crate::db::database_user::{Role, User};
use crate::system::server::TheState;
use askama::Template;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, info};

#[derive(Debug, Error)]
pub enum AdminError {
    #[error("render error: {0}")]
    Render(#[from] askama::Error),

    #[error("database error: {0}")]
    Database(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

#[derive(Template)]
#[template(path = "application/admin/admin_users_template.html")]
pub struct AdminUsersTemplate {
    pub users: Vec<User>,
    pub date: String,
    pub name_day: String,
    pub weather: String,
}

#[derive(Template)]
#[template(path = "application/admin/admin_articles_template.html")]
pub struct AdminArticlesTemplate {
    pub articles: Vec<ShortArticleData>,
    pub date: String,
    pub name_day: String,
    pub weather: String,
}

#[derive(Template)]
#[template(path = "application/admin/create_user_template.html")]
pub struct CreateUserTemplate {
    pub date: String,
    pub name_day: String,
    pub weather: String,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub author_name: String,
    pub password: String,
}

pub async fn show_admin_users(
    State(state): State<TheState>,
) -> Result<Response, AdminError> {
    debug!("show_admin_users()");
    let users = state
        .dbu
        .list_all_users()
        .await
        .map_err(|e| AdminError::Database(e.to_string()))?;

    Ok(Html(
        AdminUsersTemplate {
            users,
            date: state.ds.date(),
            name_day: state.ds.name_day(),
            weather: state.ds.weather(),
        }
        .render()?,
    )
    .into_response())
}

pub async fn handle_delete_user(
    State(state): State<TheState>,
    Path(username): Path<String>,
) -> Result<Response, AdminError> {
    debug!("handle_delete_user: {}", username);
    
    // Check if user exists and is Editor
    let user_o = state.dbu.get_user_by_name(&username).await.map_err(|e| AdminError::Database(e.to_string()))?;
    if let Some(user) = user_o {
        if user.role == Role::Editor {
            state.dbu.delete_user(&username).await.map_err(|e| AdminError::Database(e.to_string()))?;
            info!("Admin deleted user: {}", username);
        } else {
            return Err(AdminError::Database("Only Editors can be deleted".to_string()));
        }
    }

    Ok(Redirect::to("/admin/users").into_response())
}

pub async fn show_create_user_form(
    State(state): State<TheState>,
) -> Result<Response, AdminError> {
    Ok(Html(
        CreateUserTemplate {
            date: state.ds.date(),
            name_day: state.ds.name_day(),
            weather: state.ds.weather(),
        }
        .render()?,
    )
    .into_response())
}

pub async fn handle_create_user(
    State(state): State<TheState>,
    Form(payload): Form<CreateUserPayload>,
) -> Result<Response, AdminError> {
    debug!("handle_create_user: {}", payload.username);
    
    let hashed_password = hash(&payload.password, DEFAULT_COST)?;
    
    let new_user = User {
        username: payload.username,
        author_name: payload.author_name,
        password_hash: hashed_password,
        needs_password_change: true,
        role: Role::Editor,
    };
    
    state.dbu.create_user(new_user).await.map_err(|e| AdminError::Database(e.to_string()))?;
    
    Ok(Redirect::to("/admin/users").into_response())
}

pub async fn show_admin_articles(
    State(state): State<TheState>,
) -> Result<Response, AdminError> {
    debug!("show_admin_articles()");
    let articles = state
        .dba
        .list_all_articles()
        .await
        .map_err(|e| AdminError::Database(e.to_string()))?;

    Ok(Html(
        AdminArticlesTemplate {
            articles,
            date: state.ds.date(),
            name_day: state.ds.name_day(),
            weather: state.ds.weather(),
        }
        .render()?,
    )
    .into_response())
}

pub async fn handle_delete_article(
    State(state): State<TheState>,
    Path(article_file_name): Path<String>,
) -> Result<Response, AdminError> {
    debug!("handle_delete_article: {}", article_file_name);
    
    state.dba.delete_article(&article_file_name).await.map_err(|e| AdminError::Database(e.to_string()))?;
    info!("Admin deleted article: {}", article_file_name);
    
    // TODO delete the html file
    
    // TODO delete images, audio, video
    
    // Invalidate caches
    state.dv.index_invalidate();
    state.dv.news_invalidate();
    // Invalidate categories as well, to be sure
    state.dv.zahranici_invalidate();
    state.dv.republika_invalidate();
    state.dv.finance_invalidate();
    state.dv.technologie_invalidate();
    state.dv.veda_invalidate();

    Ok(Redirect::to("/admin/articles").into_response())
}
