use crate::validation::validate_media::{
    validate_data_is_audio, validate_data_is_video, MediaValidationError,
};
use image::imageops::Lanczos3;
use image::{DynamicImage, GenericImageView};
use std::fs;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown category: {0}")]
    UnknownCategory(String),

    #[error("Something wrong with the image data")]
    InvalidImageFormatError,

    #[error("Media validation error: {0}")]
    MediaValidation(#[from] MediaValidationError),
}

pub fn process_audio(audio_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    validate_data_is_audio(audio_data)?;
    save_audio(audio_data, file_name)
}

pub fn save_audio(audio_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    fs::write(format!("web/u/{}", file_name), audio_data).map_err(|e| {
        error!("Failed to save audio {}: {}", file_name, e);
        ProcessorError::Io(e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_process_audio() {
        // MP3 Magic number: ID3
        let mp3_data = [0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let file_name = "test_audio.mp3";
        let result = process_audio(&mp3_data, file_name);
        assert!(result.is_ok());
        let path = format!("web/u/{}", file_name);
        assert!(Path::new(&path).exists());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_audio() {
        let data = b"audio data";
        let result = save_audio(data, "save_audio.mp3");
        assert!(result.is_ok());
        let path = "web/u/save_audio.mp3";
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }
}
