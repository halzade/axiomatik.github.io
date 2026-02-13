use crate::data::audio_validator::AudioValidatorError;
use std::fs;
use thiserror::Error;
use AudioProcessorError::AudioIo;

#[derive(Debug, Error)]
pub enum AudioProcessorError {
    #[error("io error: {0}")]
    AudioIo(#[from] std::io::Error),

    #[error("data not recognized as audio {0}")]
    AudioValidation(#[from] AudioValidatorError),
}

pub fn process_valid_audio(audio_data: &[u8], file_name: &str) -> Result<(), AudioProcessorError> {
    save_audio_file(audio_data, file_name)
}

fn save_audio_file(audio_data: &[u8], file_name: &str) -> Result<(), AudioProcessorError> {
    fs::write(format!("web/u/{}", file_name), audio_data).map_err(AudioIo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust;
    use crate::trust::me::TrustError;
    use std::path::Path;

    #[test]
    fn test_process_audio() -> Result<(), TrustError> {
        // MP3 Magic number: ID3
        let mp3_data = [0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = process_valid_audio(&mp3_data, "test_audio.mp3");

        assert!(result.is_ok());
        assert!(Path::new("web/u/test_audio.mp3").exists());

        trust::me::remove_file("web/u/test_audio.mp3")?;

        Ok(())
    }
}
