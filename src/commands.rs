use bcrypt::{hash, DEFAULT_COST};
use axiomatik_web::db;
use tracing::{debug, error, info};

pub async fn create_user(args: &Vec<String>) {
    if args.len() != 4 {
        info!("Usage: cargo run -- create-user <username> <password>");
        std::process::exit(1);
    }

    let username = &args[2];
    let password = &args[3];

    // TODO this shouldn't init db
    let db = db::init_db().await;
    match axiomatik_web::auth::create_editor_user(&db, username, password).await {
        Ok(_) => {
            info!("Editor user '{}' created successfully.", username);
            std::process::exit(0);
        }
        Err(e) => {
            error!("Failed to create editor user: {}", e);
            std::process::exit(1);
        }
    }
}

pub async fn delete_user(args: &Vec<String>) {
    if args.len() != 3 {
        info!("Usage: cargo run -- delete-user <username>");
        std::process::exit(1);
    }

    let username = &args[2];

    // TODO
    let db = db::init_db().await;
    match db.delete_user(username).await {
        Ok(_) => {
            info!("User '{}' attempted to be deleted.", username);
            std::process::exit(0);
        }
        Err(e) => {
            error!("Failed to delete user: {}", e);
            std::process::exit(1);
        }
    }
}

pub async fn print_from_db(args: &Vec<String>) {
    if args.len() != 3 {
        info!("Usage: cargo run -- print-from-db \"<query>\"");

        for arg in args {
            debug!("Argument: {}", arg);
        }
        std::process::exit(1);
    }

    let query = &args[2];

    // TODO
    let db = db::init_db().await;
    match axiomatik_web::db_tool::print_from_db(&db, query).await {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            error!("Failed to execute query: {}", e);
            std::process::exit(1);
        }
    }
}

pub async fn create_editor_user(
    db: &Database,
    username: &str,
    password: &str,
) -> Result<(), String> {
    if password.len() < 3 {
        return Err("Heslo musí být delší".to_string());
    }

    let password_hash = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    let user = User {
        username: username.to_string(),
        author_name: username.to_string().clone(),
        password_hash,
        needs_password_change: true,
        role: Role::Editor,
    };

    db.create_user(user).await.map_err(|e| e.to_string())?;
    Ok(())
}
