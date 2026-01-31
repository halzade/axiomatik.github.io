use axum::extract::multipart::Field;
use crate::data::text_validator::validate_required_string;



pub async fn extract_required_string(field: Field<'_>) -> Result<String, UtilsError> {
    match field.text().await {
        Ok(text) => {
            validate_required_string(&text)?;
            Ok(text)
        }
        _ => Err(UtilsError::ExtractionError(name.to_string())),
    }
}

pub async fn extract_required_text(field: Field<'_>) -> Result<String, UtilsError> {
    match field.text().await {
        Ok(text) => {
            validate_required_text(&text)?;
            Ok(text)
        }
        _ => Err(UtilsError::ExtractionError(field.name.to_string())),
    }
}


pub async fn extract_optional_string(
    field: Field<'_>,
    name: &str,
) -> Result<Option<String>, UtilsError> {
    match field.text().await {
        Ok(text) => {
            validate_optional_string(&text)?;
            Ok(Some(text))
        }
        _ => Err(UtilsError::ExtractionError(name.to_string())),
    }
}


#[cfg(test)]
mod tests {

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