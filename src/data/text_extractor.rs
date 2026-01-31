use crate::data::text_validator::{
    validate_optional_string, validate_required_string, validate_required_text, TextValidationError,
};
use axum::extract::multipart::{Field, MultipartError};
use thiserror::Error;
use TextExtractorError::TextExtractionFailed;

#[derive(Debug, Error)]
pub enum TextExtractorError {
    #[error(transparent)]
    TextValidationError(#[from] TextValidationError),

    #[error("file name error")]
    TextNameError,

    #[error("file extension error")]
    TextExtensionError,

    #[error("data extension failed {0}")]
    TextExtractionFailed(#[from] MultipartError),
}

pub async fn extract_required_string(field: Field<'_>) -> Result<String, TextExtractorError> {
    match field.text().await {
        Ok(text) => {
            validate_required_string(&text)?;
            Ok(text)
        }
        Err(e) => Err(TextExtractionFailed(e)),
    }
}

pub async fn extract_required_text(field: Field<'_>) -> Result<String, TextExtractorError> {
    match field.text().await {
        Ok(text) => {
            validate_required_text(&text)?;
            Ok(text)
        }
        Err(e) => Err(TextExtractionFailed(e)),
    }
}

pub async fn extract_optional_string(
    field: Field<'_>,
) -> Result<Option<String>, TextExtractorError> {
    match field.text().await {
        Ok(text) => {
            validate_optional_string(&text)?;
            Ok(Some(text))
        }
        Err(e) => Err(TextExtractionFailed(e)),
    }
}

#[cfg(test)]
mod tests {}
