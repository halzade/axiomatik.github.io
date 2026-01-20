use axum::extract::multipart::Field;
use std::fs;
use tracing::error;
use uuid::Uuid;

pub const ALLOWED_EXTENSIONS_IMAGE: &[&str] = &["jpg", "jpeg", "png", "webp"];
pub const ALLOWED_EXTENSIONS_VIDEO: &[&str] = &["avi", "mp4", "webm", "mov"];
pub const ALLOWED_EXTENSIONS_AUDIO: &[&str] = &["mp3", "wav", "ogg", "m4a"];

pub fn validate_input(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if c.is_ascii() {
            let val = c as u32;
            // Allow printable ASCII (32-126) and common whitespace (\n, \r, \t)
            if !(val >= 32 && val <= 126 || c == '\n' || c == '\r' || c == '\t') {
                return Err("Invalid character detected");
            }
        }
        // Non-ASCII (UTF-8) is allowed
    }
    Ok(())
}

pub fn validate_search_query(input: &str) -> Result<(), &'static str> {
    if (input.len() < 3) || (input.len() > 100) {
        return Err("Input to short or too long");
    }
    for c in input.chars() {
        if c.is_ascii() {
            // No system characters (0-31, 127) and no special characters
            // Allow only alphanumeric and spaces for search
            if !c.is_ascii_alphanumeric() && c != ' ' {
                return Err("Only alphanumeric characters and spaces are allowed in search");
            }
        } else if !c.is_alphanumeric() {
            return Err("Only alphanumeric characters are allowed in search");
        }
    }
    Ok(())
}

pub fn validate_input_simple(input: &str) -> Result<(), &'static str> {
    for c in input.chars() {
        if !c.is_ascii_alphanumeric() {
            if c != '_' {
                return Err("Incorrect character detected");
            }
        }
    }
    Ok(())
}

pub fn validate_required(input: &str) -> Result<(), &'static str> {
    if input.is_empty() {
        Err("Field is required")
    } else {
        Ok(())
    }
}

pub async fn extract_text_field(field: Field<'_>, name: &str, required: bool) -> Option<String> {
    let val = match field.text().await {
        Ok(text) => text,
        Err(e) => {
            error!("Failed to get text for field '{}': {}", name, e);
            return None;
        }
    };

    if required {
        if let Err(e) = validate_required(&val) {
            error!("Validation failed for required field '{}': {}", name, e);
            return None;
        }
    }

    if let Err(e) = validate_input(&val) {
        error!("Validation failed for field '{}': {}", name, e);
        return None;
    }

    Some(val)
}

pub async fn save_file_field(field: Field<'_>, name: &str, allowed_extensions: &[&str]) -> Option<String> {
    if let Some(file_name) = field.file_name() {
        if let Err(e) = validate_input(file_name) {
            error!("Validation failed for file name '{}': {}", name, e);
            return None;
        }

        if !file_name.is_empty() {
            let extension = std::path::Path::new(file_name)
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase());

            match extension {
                Some(ext) if allowed_extensions.contains(&ext.as_str()) => {
                    let new_name = format!("{}.{}", Uuid::new_v4(), ext);
                    let data = match field.bytes().await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            error!("Failed to get bytes for field '{}': {}", name, e);
                            return None;
                        }
                    };
                    if let Err(e) = fs::write(format!("uploads/{}", new_name), data) {
                        error!("Failed to write file for field '{}': {}", name, e);
                        return None;
                    }
                    return Some(new_name);
                }
                _ => {
                    error!("{} invalid extension: {:?}", name, extension);
                    return None;
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_required() {
        assert!(validate_required("").is_err());
        assert!(validate_required("x").is_ok());
    }

    #[test]
    fn test_validate_input() {
        assert!(validate_input("").is_ok());
        assert!(validate_input("Hello\nWorld\r\t").is_ok());
        assert!(validate_input("Příliš žluťoučký kůň úpěl ďábelské ódy").is_ok()); // Non-ASCII UTF-8 is allowed
        assert!(validate_input("Hello \x01 World").is_err()); // ASCII control character
        assert!(validate_input("Hello \x7F World").is_err()); // ASCII DEL
    }

    #[test]
    fn test_validate_search_query() {
        assert!(validate_search_query("").is_err());
        assert!(validate_search_query("Hello World").is_ok());
        assert!(validate_search_query("Hello123").is_ok());
        assert!(validate_search_query("Příliš").is_ok()); // Non-ASCII alphanumeric is allowed
        assert!(validate_search_query("Hello!").is_err()); // Special character
        assert!(validate_search_query("Hello\nWorld").is_err()); // Whitespace other than space
    }

    #[test]
    fn test_validate_input_simple() {
        assert!(validate_input_simple("").is_ok());
        assert!(validate_input_simple("Hello_World123").is_ok());
        assert!(validate_input_simple("Hello World").is_err()); // Space is not allowed
        assert!(validate_input_simple("Příliš").is_err()); // Non-ASCII is not allowed
        assert!(validate_input_simple("Hello-World").is_err()); // Hyphen is not allowed
    }

    use axum::body::Body;
    use axum::extract::{FromRequest, Multipart};
    use http::Request;

    #[tokio::test]
    async fn test_extract_text_field() {
        let boundary = "boundary";
        let body = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"title\"\r\n\r\n\
             My Title\r\n\
             --{boundary}--\r\n"
        );
        let req = Request::builder()
            .header("Content-Type", format!("multipart/form-data; boundary={boundary}"))
            .body(Body::from(body))
            .unwrap();

        let mut multipart = Multipart::from_request(req, &()).await.expect("Failed to create Multipart");
        let field = multipart.next_field().await.expect("Failed to get next field").expect("No field found");

        let val = extract_text_field(field, "title", true).await;
        assert_eq!(val, Some("My Title".to_string()));
    }

    #[tokio::test]
    async fn test_extract_text_field_required_fail() {
        let boundary = "boundary";
        let body = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"title\"\r\n\r\n\
             \r\n\
             --{boundary}--\r\n"
        );
        let req = Request::builder()
            .header("Content-Type", format!("multipart/form-data; boundary={boundary}"))
            .body(Body::from(body))
            .unwrap();

        let mut multipart = Multipart::from_request(req, &()).await.expect("Failed to create Multipart");
        let field = multipart.next_field().await.expect("Failed to get next field").expect("No field found");

        let val = extract_text_field(field, "title", true).await;
        assert_eq!(val, None);
    }

    #[tokio::test]
    async fn test_save_file_field() {
        let boundary = "boundary";
        let body = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\n\
             Content-Type: image/jpeg\r\n\r\n\
             fake_image_data\r\n\
             --{boundary}--\r\n"
        );
        let req = Request::builder()
            .header("Content-Type", format!("multipart/form-data; boundary={boundary}"))
            .body(Body::from(body))
            .unwrap();

        let mut multipart = Multipart::from_request(req, &()).await.expect("Failed to create Multipart");
        let field = multipart.next_field().await.expect("Failed to get next field").expect("No field found");

        // Ensure uploads directory exists for test
        let _ = fs::create_dir_all("uploads");

        let val = save_file_field(field, "image", ALLOWED_EXTENSIONS_IMAGE).await;
        assert!(val.is_some());
        let saved_path = format!("uploads/{}", val.unwrap());
        assert!(std::path::Path::new(&saved_path).exists());

        // Cleanup
        let _ = fs::remove_file(saved_path);
    }

    #[tokio::test]
    async fn test_save_file_field_invalid_extension() {
        let boundary = "boundary";
        let body = format!(
            "--{boundary}\r\n\
             Content-Disposition: form-data; name=\"image\"; filename=\"test.exe\"\r\n\
             Content-Type: application/x-msdownload\r\n\r\n\
             fake_data\r\n\
             --{boundary}--\r\n"
        );
        let req = Request::builder()
            .header("Content-Type", format!("multipart/form-data; boundary={boundary}"))
            .body(Body::from(body))
            .unwrap();

        let mut multipart = Multipart::from_request(req, &()).await.expect("Failed to create Multipart");
        let field = multipart.next_field().await.expect("Failed to get next field").expect("No field found");

        let val = save_file_field(field, "image", ALLOWED_EXTENSIONS_IMAGE).await;
        assert!(val.is_none());
    }
}
