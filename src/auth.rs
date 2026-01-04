use crate::db::{Database, User, Role};
use bcrypt::{DEFAULT_COST, hash, verify};

pub async fn authenticate_user(
    db: &Database,
    username: &str,
    password: &str,
) -> Result<User, String> {
    match db.get_user(username).await {
        Ok(Some(user)) => {
            if verify(password, &user.password_hash).unwrap_or(false) {
                Ok(user)
            } else {
                Err("Invalid password".to_string())
            }
        }
        Ok(None) => Err("User not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub async fn create_editor_user(
    db: &Database,
    username: &str,
    password: &str,
) -> Result<(), String> {
    if password.len() < 5 {
        return Err("Password must be at least 5 characters long".to_string());
    }

    let password_hash = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    let user = User {
        username: username.to_string(),
        password_hash,
        needs_password_change: false,
        role: Role::Editor,
    };

    db.create_user(user).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn change_password(
    db: &Database,
    username: &str,
    new_password: &str,
) -> Result<(), String> {
    if new_password.len() < 8 {
        return Err("Heslo musí mít alespoň 8 znaků".to_string());
    }

    match db.get_user(username).await {
        Ok(Some(mut user)) => {
            let password_hash = hash(new_password, DEFAULT_COST).unwrap();
            user.password_hash = password_hash;
            user.needs_password_change = false;
            db.update_user(user).await.map_err(|e| e.to_string())?;
            Ok(())
        }
        Ok(None) => Err("User not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}
