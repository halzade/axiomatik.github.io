use thiserror::Error;

#[derive(Debug, Error)]
pub enum MediaValidationError {
    #[error("No audio stream found")]
    NoAudioStream,

    #[error("No video stream found")]
    NoVideoStream,

    #[error("Unexpected media type: {0}")]
    UnexpectedMedia(String),

    #[error("Could not determine media type")]
    UnknownType,
}

/// Validate that provided data is **audio-only**
pub fn validate_data_is_audio(data: &[u8]) -> Result<(), MediaValidationError> {
    let kind = infer::get(data).ok_or(MediaValidationError::UnknownType)?;

    match kind.matcher_type() {
        infer::MatcherType::Audio => Ok(()),
        _ => Err(MediaValidationError::UnexpectedMedia(kind.mime_type().to_string())),
    }
}

/// Validate that provided data is **video (may include audio)**
pub fn validate_data_is_video(data: &[u8]) -> Result<(), MediaValidationError> {
    let kind = infer::get(data).ok_or(MediaValidationError::UnknownType)?;

    match kind.matcher_type() {
        infer::MatcherType::Video => Ok(()),
        _ => Err(MediaValidationError::UnexpectedMedia(kind.mime_type().to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_audio() {
        // MP3 Magic number: ID3 or 0xFF 0xFB
        let mp3_data = [0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert!(validate_data_is_audio(&mp3_data).is_ok());

        let random_data = [0x00, 0x01, 0x02, 0x03];
        assert!(validate_data_is_audio(&random_data).is_err());
    }

    #[test]
    fn test_validate_video() {
        // MP4 Magic number (ftyp)
        let mp4_data = [0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d];
        assert!(validate_data_is_video(&mp4_data).is_ok());

        let mp3_data = [0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert!(validate_data_is_video(&mp3_data).is_err());
    }
}