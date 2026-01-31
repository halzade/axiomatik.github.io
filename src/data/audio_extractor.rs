use crate::data::audio_extractor::AudioExtractorError::{DetectedEmptyFile, UnsupportedFileType};
use axum::extract::multipart::{Field, MultipartError};
use thiserror::Error;
use AudioExtractorError::{DataExtractionFailed, FileExtensionError, FileNameError};

const ALLOWED_EXTENSIONS_AUDIO: &[&str] = &["mp3", "wav", "ogg", "m4a"];

#[derive(Debug, Error)]
pub enum AudioExtractorError {
    #[error("unsupported format {0}")]
    UnsupportedFileType(String),

    #[error("empty file was detected")]
    DetectedEmptyFile,

    #[error("file name error")]
    FileNameError,

    #[error("file extension error")]
    FileExtensionError,

    #[error("data extension failed {0}")]
    DataExtractionFailed(#[from] MultipartError),
}

pub async fn extract_audio_data(
    field: Field<'_>,
) -> Result<(Vec<u8>, String), AudioExtractorError> {
    // extension
    let file_name = field.file_name().ok_or(FileNameError)?.to_string();
    let ext = file_name
        .split('.')
        .last()
        .ok_or(FileExtensionError)?
        .to_lowercase();
    // data
    let bytes = field
        .bytes()
        .await
        .map_err(|e| DataExtractionFailed(e))?
        .to_vec();
    Ok((bytes, ext))
}

fn validate_audio_extension(ext: &str) -> Result<(), AudioExtractorError> {
    if !ALLOWED_EXTENSIONS_AUDIO.contains(&ext) {
        return Err(UnsupportedFileType(ext.into()));
    }
    Ok(())
}

fn validate_audio_data(bytes: Vec<u8>) -> Result<(), AudioExtractorError> {
    if bytes.is_empty() {
        Err(DetectedEmptyFile)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::audio_extractor::{validate_audio_data, validate_audio_extension};

    #[test]
    fn test_validate_audio_extension() {
        assert!(validate_audio_extension("mp3").is_ok());
        assert!(validate_audio_extension("mp4").is_err());
    }

    #[test]
    fn test_validate_audio_data() {
        assert!(validate_audio_data(vec![1, 2, 3]).is_ok());
        assert!(validate_audio_data(Vec::new()).is_err());
    }
}
