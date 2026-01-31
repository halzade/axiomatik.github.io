use crate::data::audio_validator::{validate_data_is_audio, AudioValidatorError};
use std::fs;
use thiserror::Error;
use tracing::error;
use AudioProcessorError::AudioIo;

#[derive(Debug, Error)]
pub enum AudioProcessorError {
    #[error("io error: {0}")]
    AudioIo(#[from] std::io::Error),

    #[error("data not recognized as audio {0}")]
    AudioValidation(#[from] AudioValidatorError),
}

pub fn process_audio(audio_data: &[u8], file_name: &str) -> Result<(), AudioProcessorError> {
    validate_data_is_audio(audio_data)?;
    save_audio(audio_data, file_name)
}

fn save_audio(audio_data: &[u8], file_name: &str) -> Result<(), AudioProcessorError> {
    fs::write(format!("web/u/{}", file_name), audio_data).map_err(|e| AudioIo(e))
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
        let result = process_audio(&mp3_data, "test_audio.mp3");
        assert!(result.is_ok());
        assert!(Path::new("web/u/test_audio.mp3").exists());

        fs::remove_file("web/u/test_audio.mp3").unwrap();
    }

    fn test_process_audio_err() {
        let fake_data = b"fake data";
        let result = process_audio(fake_data, "fake_audio.mp3");
        assert!(result.is_err());
        assert_ne!(Path::new("web/u/fake_audio.mp3").exists(), false);
    }
}
