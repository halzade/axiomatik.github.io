use crate::data::video_extractor::VideoExtractorError::{
    VideoExtensionError, VideoExtractionFailed, VideoNameError,
};
use axum::extract::multipart::{Field, MultipartError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VideoExtractorError {
    #[error("file name error")]
    VideoNameError,

    #[error("file extension error")]
    VideoExtensionError,

    #[error("data extension failed {0}")]
    VideoExtractionFailed(#[from] MultipartError),
}

pub async fn extract_video_data(
    field: Field<'_>,
) -> Result<(Vec<u8>, String), VideoExtractorError> {
    // extension
    let file_name = field.file_name().ok_or(VideoNameError)?.to_string();
    let ext = file_name
        .split('.')
        .last()
        .ok_or(VideoExtensionError)?
        .to_lowercase();
    // data
    let bytes = field
        .bytes()
        .await
        .map_err(|e| VideoExtractionFailed(e))?
        .to_vec();
    Ok((bytes, ext))
}
