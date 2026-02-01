use crate::data::image_validator::ImageValidationError::{ImageWidthValidation, UnknownDataType};
use image::DynamicImage;
use thiserror::Error;

const ALLOWED_EXTENSIONS_IMAGE: &[&str] = &["jpg", "jpeg", "png"];

#[derive(Debug, Error)]
pub enum ImageValidationError {
    #[error("validation error: {0}")]
    ImageValidation(String),

    #[error("width {0} is less than 820")]
    ImageWidthValidation(u32),

    #[error("unrecognized data type {0}")]
    UnknownDataType(String),

    #[error("undefined data type")]
    UndefinedDataType,
}

pub fn validate_image_data(img: &DynamicImage) -> Result<(), ImageValidationError> {
    // TODO X
    Ok(())
}

pub fn validate_image_width(width: u32) -> Result<(), ImageValidationError> {
    if width < 820 {
        return Err(ImageWidthValidation(width));
    }
    Ok(())
}

fn validate_image_extension(ext: &str) -> Result<(), ImageValidationError> {
    if !ALLOWED_EXTENSIONS_IMAGE.contains(&ext) {
        return Err(UnknownDataType(ext.into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::data::image_validator::validate_image_extension;

    #[test]
    fn test_validate_image_extension() {
        assert!(validate_image_extension("jpg").is_ok());
        assert!(validate_image_extension("gif").is_err());
    }
}
