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

pub fn process_video(video_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    validate_data_is_video(video_data)?;
    save_video(video_data, file_name)
}

pub fn save_video(video_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    fs::write(format!("web/u/{}", file_name), video_data).map_err(|e| {
        error!("Failed to save video {}: {}", file_name, e);
        ProcessorError::Io(e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_process_video() {
        // MP4 Magic number (ftyp)
        let mp4_data = [
            0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d,
        ];
        let file_name = "test_video.mp4";
        let result = process_video(&mp4_data, file_name);
        assert!(result.is_ok());
        let path = format!("web/u/{}", file_name);
        assert!(Path::new(&path).exists());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_video() {
        let data = b"video data";
        let result = save_video(data, "save_video.mp4");
        assert!(result.is_ok());
        let path = "web/u/save_video.mp4";
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }
}
