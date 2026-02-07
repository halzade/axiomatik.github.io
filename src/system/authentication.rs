use crate::db::database_user::{DatabaseUser, User};
use std::convert::Infallible;
use tracing::debug;

/**
 * user authentication
 */
#[derive(Clone, Debug)]
pub struct Backend;

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
            Ok(user_o) => {
                match user_o {
                    Some(user) => {
                        if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
                            /*
                             * user was authenticated
                             */
                            return Ok(Some(user));
                        }
                        Ok(None)
                    }
                    None => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    async fn get_user(
        &self,
        user_name: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        // TODO from state
        let db_user_r = DatabaseUser::new_from_scratch().await;
        match db_user_r {
            Ok(db_user) => {
                let user_ro = db_user.get_user_by_name(user_name).await;
                match user_ro {
                    Ok(user_o) => match user_o {
                        Some(user) => Ok(Some(user)),
                        None => Ok(None),
                    },
                    Err(_) => Ok(None),
                }
            }
            Err(_) => Ok(None),
        }
    }
}
