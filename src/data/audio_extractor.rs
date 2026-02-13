use axum::extract::multipart::{Field, MultipartError};
use thiserror::Error;
use AudioExtractorError::{AudioExtensionError, AudioExtractionFailed, AudioNameError};

#[derive(Debug, Error)]
pub enum AudioExtractorError {
    #[error("file name error")]
    AudioNameError,

    #[error("file extension error")]
    AudioExtensionError,

    #[error("data extension failed {0}")]
    AudioExtractionFailed(#[from] MultipartError),
}

pub async fn extract_audio_data(
    field: Field<'_>,
) -> Result<(Vec<u8>, String), AudioExtractorError> {
    // extension
    let file_name = field.file_name().ok_or(AudioNameError)?.to_string();
    let ext = file_name
        .split('.')
        .next_back()
        .ok_or(AudioExtensionError)?
        .to_lowercase();
    // data
    let bytes = field
        .bytes()
        .await
        .map_err(AudioExtractionFailed)?
        .to_vec();
    Ok((bytes, ext))
}

#[cfg(test)]
mod tests {}
