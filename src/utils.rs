use crate::validation::{
    validate_optional_string, validate_required_string, validate_required_text,
};
use anyhow::{anyhow, Error};
use axum::extract::multipart::Field;

const ALLOWED_EXTENSIONS_IMAGE: &[&str] = &["jpg", "jpeg", "png"];
const ALLOWED_EXTENSIONS_VIDEO: &[&str] = &["avi", "mp4"];
const ALLOWED_EXTENSIONS_AUDIO: &[&str] = &["mp3", "wav", "ogg", "m4a"];

pub async fn extract_required_string(field: Field<'_>, name: &str) -> Result<String, Error> {
    match field.text().await {
        Ok(text) => {
            // extracted
            match validate_required_string(&text) {
                Ok(_) => {
                    // sanitized & validated
                    Ok(text)
                }
                _ => Err(anyhow!(format!("Invalid input value for: {}", name))),
            }
        }
        _ => Err(anyhow!(format!("Error extracting: {}", name))),
    }
}

pub async fn extract_required_text(field: Field<'_>, name: &str) -> Result<String, Error> {
    match field.text().await {
        Ok(text) => {
            // extracted
            match validate_required_text(&text) {
                Ok(_) => {
                    // sanitized & validated
                    Ok(text)
                }
                _ => Err(anyhow!(format!("Invalid input value for: {}", name))),
            }
        }
        _ => Err(anyhow!(format!("Error extracting: {}", name))),
    }
}

pub async fn extract_optional_string(
    field: Field<'_>,
    name: &str,
) -> Result<Option<String>, Error> {
    match field.text().await {
        Ok(text) => {
            // extracted
            match validate_optional_string(&text) {
                Ok(_) => {
                    // sanitized & validated
                    Ok(Some(text))
                }
                _ => Err(anyhow!(format!("Invalid input value for: {}", name))),
            }
        }
        _ => Err(anyhow!(format!("Error extracting: {}", name))),
    }
}

pub async fn extract_image_data(field: Field<'_>) -> Option<(Vec<u8>, String)> {
    let file_name = field.file_name()?.to_string();
    let bytes = field.bytes().await.ok()?.to_vec();
    validate_and_extract(&file_name, bytes, ALLOWED_EXTENSIONS_IMAGE)
}

pub async fn extract_video_data(field: Field<'_>) -> Option<(Vec<u8>, String)> {
    let file_name = field.file_name()?.to_string();
    let bytes = field.bytes().await.ok()?.to_vec();
    validate_and_extract(&file_name, bytes, ALLOWED_EXTENSIONS_VIDEO)
}

pub async fn extract_audio_data(field: Field<'_>) -> Option<(Vec<u8>, String)> {
    let file_name = field.file_name()?.to_string();
    let bytes = field.bytes().await.ok()?.to_vec();
    validate_and_extract(&file_name, bytes, ALLOWED_EXTENSIONS_AUDIO)
}

fn validate_and_extract(
    file_name: &str,
    bytes: Vec<u8>,
    allowed_extensions: &[&str],
) -> Option<(Vec<u8>, String)> {
    let extension = file_name.split('.').last()?.to_lowercase();
    if !allowed_extensions.contains(&extension.as_str()) {
        return None;
    }

    if bytes.is_empty() {
        None
    } else {
        Some((bytes, extension))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_image_data() {
        let allowed = ALLOWED_EXTENSIONS_IMAGE;
        let data = vec![1, 2, 3];
        assert_eq!(validate_and_extract("t.jpg", data.clone(), allowed), Some((data.clone(), "jpg".to_string())));
        assert_eq!(validate_and_extract("t.gif", data.clone(), allowed), None);
    }

    #[test]
    fn test_extract_video_data() {
        let allowed = ALLOWED_EXTENSIONS_VIDEO;
        let data = vec![1, 2, 3];
        assert_eq!(validate_and_extract("t.mp4", data.clone(), allowed), Some((data.clone(), "mp4".to_string())));
        assert_eq!(validate_and_extract("t.jpg", data.clone(), allowed), None);
    }

    #[test]
    fn test_extract_audio_data() {
        let allowed = ALLOWED_EXTENSIONS_AUDIO;
        let data = vec![1, 2, 3];
        assert_eq!(validate_and_extract("t.mp3", data.clone(), allowed), Some((data.clone(), "mp3".to_string())));
        assert_eq!(validate_and_extract("t.mp4", data.clone(), allowed), None);
    }

    #[test]
    fn test_extract_required_string() {
        assert!(validate_required_string("valid").is_ok());
        assert!(validate_required_string("").is_err());
    }

    #[test]
    fn test_extract_required_text() {
        assert!(validate_required_text("valid text").is_ok());
        assert!(validate_required_text("").is_err());
    }

    #[test]
    fn test_extract_optional_string() {
        assert!(validate_optional_string("valid").is_ok());
        assert!(validate_optional_string("").is_ok());
    }
}
