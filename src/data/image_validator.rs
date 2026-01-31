use crate::data::image_validator::ImageValidationError::ImageWidthValidation;
use image::{DynamicImage, GenericImageView, ImageError};
use thiserror::Error;

const ALLOWED_EXTENSIONS_IMAGE: &[&str] = &["jpg", "jpeg", "png"];

#[derive(Debug, Error)]
pub enum ImageValidationError {
    #[error("failed to save image because {0}")]
    ImageFormat(#[from] ImageError),

    #[error("validation error: {0}")]
    ImageValidation(String),

    #[error("width {0} is less than 820")]
    ImageWidthValidation(u32),
}

fn validate_data_is_image() {
    // TODO
}

fn validate_image_width(img: &DynamicImage) -> Result<(), ImageValidationError> {
    let (width, _) = img.dimensions();
    if width < 820 {
        Err(ImageWidthValidation(width))
    }
    Ok(())
}
