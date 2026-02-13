use crate::data::video_validator::VideoValidatorError::{
    DetectedEmptyVideoFile, UndefinedVideoType, UnknownVideoType, UnsupportedVideoType,
};
use infer::MatcherType::Video;
use thiserror::Error;

const ALLOWED_EXTENSIONS_VIDEO: &[&str] = &["avi", "mp4"];

#[derive(Debug, Error)]
pub enum VideoValidatorError {
    #[error("unrecognized data type {0}")]
    UnknownVideoType(String),

    #[error("undefined data type")]
    UndefinedVideoType,

    #[error("unsupported format {0}")]
    UnsupportedVideoType(String),

    #[error("detected empty video file")]
    DetectedEmptyVideoFile,
}

pub fn validate_video_data(data: &[u8]) -> Result<(), VideoValidatorError> {
    if data.is_empty() {
        return Err(DetectedEmptyVideoFile);
    }
    infer::get(data).map_or(Err(UndefinedVideoType), |kind| match kind.matcher_type() {
        Video => Ok(()),
        _ => Err(UnknownVideoType(kind.to_string())),
    })
}

pub fn validate_video_extension(ext: &str) -> Result<(), VideoValidatorError> {
    if !ALLOWED_EXTENSIONS_VIDEO.contains(&ext) {
        return Err(UnsupportedVideoType(ext.into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::data::video_validator::{validate_video_data, validate_video_extension};

    #[test]
    fn test_validate_video() {
        // MP4 Magic number (partial): ....ftypisom
        let mp4_data = [
            0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d,
        ];
        assert!(validate_video_data(&mp4_data).is_ok());

        let random_data = [0x00, 0x01, 0x02, 0x03];
        assert!(validate_video_data(&random_data).is_err());
    }

    #[test]
    fn test_validate_video_extension() {
        assert!(validate_video_extension("mp4").is_ok());
        assert!(validate_video_extension("avi").is_ok());
        assert!(validate_video_extension("mp3").is_err());
    }
}

