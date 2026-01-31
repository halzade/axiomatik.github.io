use crate::data::audio_validator::AudioValidatorError::UnknownDataType;
use infer::MatcherType::Audio;
use thiserror::Error;
use AudioValidatorError::UndefinedDataType;

#[derive(Debug, Error)]
pub enum AudioValidatorError {
    #[error("unrecognized data type {0}")]
    UnknownDataType(String),

    #[error("undefined data type")]
    UndefinedDataType,
}
pub fn validate_data_is_audio(data: &[u8]) -> Result<(), AudioValidatorError> {
    match infer::get(data) {
        Some(kind) => match kind.matcher_type() {
            Audio => Ok(()),
            _ => Err(UnknownDataType(kind.to_string())),
        },
        None => Err(UndefinedDataType),
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
}
