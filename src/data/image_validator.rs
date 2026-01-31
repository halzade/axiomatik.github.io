use crate::data::image_validator::ImageValidationError::ImageWidthValidation;
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
    // TODO

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
        return Err(UnsupportedFileType(ext.into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_image_data() {
        let data = vec![1, 2, 3];
        assert_eq!(validate_and_extract("t.jpg", data.clone()), Some((data.clone(), "jpg".to_string())));
        assert_eq!(validate_and_extract("t.gif", data.clone()), None);
    }
}