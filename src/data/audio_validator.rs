use crate::data::audio_validator::AudioValidatorError::{
    DetectedEmptyAudioFile, UndefinedAudioType, UnknownAudioType, UnsupportedAudioType,
};
use infer::MatcherType::Audio;
use thiserror::Error;

const ALLOWED_EXTENSIONS_AUDIO: &[&str] = &["mp3", "wav", "ogg", "m4a"];

#[derive(Debug, Error)]
pub enum AudioValidatorError {
    #[error("unrecognized data type {0}")]
    UnknownAudioType(String),

    #[error("undefined data type")]
    UndefinedAudioType,

    #[error("unsupported format {0}")]
    UnsupportedAudioType(String),

    #[error("detected empty audio file")]
    DetectedEmptyAudioFile,
}

pub fn validate_audio_data(data: &[u8]) -> Result<(), AudioValidatorError> {
    if data.is_empty() {
        return Err(DetectedEmptyAudioFile);
    }
    match infer::get(data) {
        Some(kind) => match kind.matcher_type() {
            Audio => Ok(()),
            _ => Err(UnknownAudioType(kind.to_string())),
        },
        None => Err(UndefinedAudioType),
    }
}

pub fn validate_audio_extension(ext: &str) -> Result<(), AudioValidatorError> {
    if !ALLOWED_EXTENSIONS_AUDIO.contains(&ext) {
        return Err(UnsupportedAudioType(ext.into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::data::audio_validator::{validate_audio_data, validate_audio_extension};

    #[test]
    fn test_validate_audio() {
        // MP3 Magic number: ID3 or 0xFF 0xFB
        let mp3_data = [0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert!(validate_audio_data(&mp3_data).is_ok());

        let random_data = [0x00, 0x01, 0x02, 0x03];
        assert!(validate_audio_data(&random_data).is_err());
    }

    #[test]
    fn test_validate_audio_extension() {
        assert!(validate_audio_extension("mp3").is_ok());
        assert!(validate_audio_extension("mp4").is_err());
    }
}
