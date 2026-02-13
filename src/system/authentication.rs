use crate::db::database_user::{DatabaseUser, User};
use std::convert::Infallible;
use std::sync::Arc;
use tracing::debug;

/**
 * user authentication
 */
#[derive(Clone, Debug)]
pub struct Backend {
    pub db_user: Arc<DatabaseUser>,
}

impl axum_login::AuthnBackend for Backend {
    type User = User;
    type Credentials = (String, String);
    type Error = Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let (username, password) = creds;

        debug!("Authenticating user {:?}", username);
        let user_r = self.get_user(&username).await;
        match user_r {
            Ok(Some(user)) => {
                if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
                    /*
                     * user was authenticated
                     */
                    return Ok(Some(user));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    async fn get_user(
        &self,
        user_name: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user_ro = self.db_user.get_user_by_name(user_name).await;
        user_ro.map_or(Ok(None), Ok)
    }
}
