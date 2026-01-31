use infer::MatcherType::Video;
use std::fs;
use thiserror::Error;
use tracing::error;
use VideoProcessorError::{UnexpectedMedia, UnknownType};

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
}

pub fn process_video(video_data: &[u8], file_name: &str) -> Result<(), VideoProcessorError> {
    validate_data_is_video(video_data)?;

    // TODO validate there's not audio

    save_video(video_data, file_name)
}

/// Validate that provided data is **video (may include audio)**
fn validate_data_is_video(data: &[u8]) -> Result<(), VideoProcessorError> {
    let kind = infer::get(data).ok_or(UnknownType)?;
    match kind.matcher_type() {
        Video => Ok(()),
        _ => Err(UnexpectedMedia(kind.mime_type().to_string())),
    }
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
    fn test_validate_video() {
        // MP4 Magic number (ftyp)
        let mp4_data = [
            0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d,
        ];
        assert!(validate_data_is_video(&mp4_data).is_ok());

        let mp3_data = [0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert!(validate_data_is_video(&mp3_data).is_err());
    }

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
