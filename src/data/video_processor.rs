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
    validate_video_data(video_data)?;
    verify_no_audio(video_data)?;

    save_video(video_data, file_name)
}

fn verify_no_audio(video_data: &[u8]) -> Result<(), VideoProcessorError> {
    // In MP4 files, audio tracks are identified by the 'soun' handler type in the 'hdlr' box.
    // In AVI files, audio streams are identified by the 'auds' stream type in the 'strh' header.

    // Check for 'soun' (MP4 audio handler)
    if contains_marker(video_data, b"soun") {
        return Err(VideoProcessorError::AudioStreamFound);
    }

    // Check for 'auds' (AVI audio stream)
    if contains_marker(video_data, b"auds") {
        return Err(VideoProcessorError::AudioStreamFound);
    }

    Ok(())
}

fn contains_marker(data: &[u8], marker: &[u8]) -> bool {
    data.windows(marker.len()).any(|window| window == marker)
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
    fn test_process_video_with_audio_mp4() {
        // MP4 Magic number (ftyp) + 'soun' marker
        let mut mp4_data = vec![
            0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d,
        ];
        mp4_data.extend_from_slice(b"some extra data soun more data");

        let file_name = "test_video_audio.mp4";
        let result = process_video(&mp4_data, file_name);
        assert!(matches!(result, Err(VideoProcessorError::AudioStreamFound)));
    }

    #[test]
    fn test_process_video_with_audio_avi() {
        // MP4 Magic number (to pass validate_video_data) + 'auds' marker
        let mut data = vec![
            0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d,
        ];
        data.extend_from_slice(b"some extra data auds more data");

        let file_name = "test_video_audio.avi";
        let result = process_video(&data, file_name);
        assert!(matches!(result, Err(VideoProcessorError::AudioStreamFound)));
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
