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
    extract_data(field, file_name, ALLOWED_EXTENSIONS_IMAGE).await
}

pub async fn extract_video_data(field: Field<'_>) -> Option<(Vec<u8>, String)> {
    let file_name = field.file_name()?.to_string();
    extract_data(field, file_name, ALLOWED_EXTENSIONS_VIDEO).await
}

pub async fn extract_audio_data(field: Field<'_>) -> Option<(Vec<u8>, String)> {
    let file_name = field.file_name()?.to_string();
    extract_data(field, file_name, ALLOWED_EXTENSIONS_AUDIO).await
}

async fn extract_data(
    field: Field<'_>,
    file_name: String,
    allowed_extensions: &[&str],
) -> Option<(Vec<u8>, String)> {
    let extension = file_name.split('.').last()?.to_lowercase();

    if !allowed_extensions.contains(&extension.as_str()) {
        return None;
    }

    match field.bytes().await {
        Ok(bytes) => {
            if bytes.is_empty() {
                None
            } else {
                Some((bytes.to_vec(), extension))
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_framework::article_builder::ArticleBuilder;
    use crate::test_framework::script_base;
    use crate::test_framework::script_base_data::PNG;

    #[test]
    fn test_extract_image_data() {
        let image_data = script_base::get_test_image_data();
        let article = ArticleBuilder::new()
            .image("test.jpg", &image_data, PNG)
            .build();

        // extract_image_data()
    }

    #[test]
    fn test_extract_video_data() {}

    #[test]
    fn test_extract_audio_data() {}

    #[test]
    fn test_extract_required_string() {}

    #[test]
    fn test_extract_required_text() {}

    #[test]
    fn test_extract_optional_string() {}
}
