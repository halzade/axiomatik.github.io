use crate::data::image_extractor::ImageExtractorError::{
    ImageExtensionError, ImageExtractionFailed, ImageNameError,
};
use axum::extract::multipart::{Field, MultipartError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageExtractorError {
    #[error("file name error")]
    ImageNameError,

    #[error("file extension error")]
    ImageExtensionError,

    #[error("data extension failed {0}")]
    ImageExtractionFailed(#[from] MultipartError),
}

pub async fn extract_image_data(
    field: Field<'_>,
) -> Result<(Vec<u8>, String), ImageExtractorError> {
    // extension
    let file_name = field.file_name().ok_or(ImageNameError)?.to_string();
    let ext = file_name
        .split('.')
        .next_back()
        .ok_or(ImageExtensionError)?
        .to_lowercase();
    // data
    let bytes = field
        .bytes()
        .await
        .map_err(ImageExtractionFailed)?
        .to_vec();
    Ok((bytes, ext))
}
