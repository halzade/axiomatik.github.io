use crate::data::video_validator::{validate_video_data, VideoValidatorError};
use std::fs;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum VideoProcessorError {
    #[error("Something wrong with the image data")]
    Io(#[from] std::io::Error),

    #[error("Video image must not contain audio stream")]
    AudioStreamFound,

    #[error("No video stream found")]
    NoVideoStream,

    #[error("Unexpected media type: {0}")]
    UnexpectedMedia(String),

    #[error("Could not determine media type")]
    UnknownType,

    #[error("Video validation error: {0}")]
    ValidationError(#[from] VideoValidatorError),
}

pub fn process_video(video_data: &[u8], file_name: &str) -> Result<(), VideoProcessorError> {
    // TODO X validate there's no audio
    validate_video_data(video_data)?;

    save_video(video_data, file_name)
}

fn save_video(video_data: &[u8], file_name: &str) -> Result<(), VideoProcessorError> {
    fs::write(format!("web/u/{}", file_name), video_data).map_err(|e| {
        error!("Failed to save video {}: {}", file_name, e);
        VideoProcessorError::Io(e)
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
        let path = "web/u/test_video.mp4";
        assert!(Path::new(path).exists());
        assert!(fs::remove_file(path).is_ok());
    }

    #[test]
    fn test_save_video() {
        let data = b"video data";
        let result = save_video(data, "save_video.mp4");
        assert!(result.is_ok());
        let path = "web/u/save_video.mp4";
        assert!(Path::new(path).exists());
        assert!(fs::remove_file(path).is_ok());
    }
}
