use crate::validation::validate_media::{
    validate_data_is_audio, validate_data_is_video, MediaValidationError,
};
use image::imageops::Lanczos3;
use image::{DynamicImage, GenericImageView};
use std::fs;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("Something wrong with the image data")]
    InvalidImageFormatError,

    #[error("Media validation error: {0}")]
    MediaValidation(#[from] MediaValidationError),
}

pub fn process_images(img: &DynamicImage, new_name: &str, ext: &str) -> Option<()> {
    let (width, height) = img.dimensions();
    if width < 820 {
        error!("Image width {} is less than 820px", width);
        return None;
    }

    // Save 820xheight
    let img_820 = img.resize(820, height, Lanczos3);
    let base_name = new_name.split('.').next().unwrap();
    let name_820 = format!("{}_image_820.{}", base_name, ext);

    // Save 820xany
    let _ = save_image(&img_820, name_820.as_str());

    // Save 50x50
    resized_and_save_image(&img, 50, 50, base_name, "image_50", &ext);
    // Save 288x211
    resized_and_save_image(&img, 288, 211, base_name, "image_288", &ext);
    // Save 440x300
    resized_and_save_image(&img, 440, 300, base_name, "image_440", &ext);

    Some(())
}

pub fn resized_and_save_image(
    img: &DynamicImage,
    w: u32,
    h: u32,
    base_name: &str,
    suffix: &str,
    ext: &str,
) {
    let resized = img.resize_to_fill(w, h, Lanczos3);
    let name = format!("{}_{}.{}", base_name, suffix, ext);
    let _ = save_image(&resized, name.as_str());
}

pub fn save_image(image: &DynamicImage, file_name: &str) -> Result<(), ProcessorError> {
    image.save(format!("web/u/{}", file_name)).map_err(|e| {
        error!("Failed to save image {}: {}", file_name, e);
        ProcessorError::InvalidImageFormatError
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbImage};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_process_images() {
        let img = DynamicImage::ImageRgb8(RgbImage::new(1000, 1000));
        let result = process_images(&img, "test_image.png", "png");
        assert!(result.is_some());

        let expected_files = [
            "web/u/test_image_image_820.png",
            "web/u/test_image_image_50.png",
            "web/u/test_image_image_288.png",
            "web/u/test_image_image_440.png",
        ];

        for file in expected_files.iter() {
            assert!(Path::new(file).exists(), "File {} does not exist", file);
            fs::remove_file(file).unwrap();
        }
    }

    #[test]
    fn test_process_images_too_small() {
        let img = DynamicImage::ImageRgb8(RgbImage::new(100, 100));
        let result = process_images(&img, "test_small.png", "png");
        assert!(result.is_none());
    }

    #[test]
    fn test_resized_and_save_image() {
        let img = DynamicImage::ImageRgb8(RgbImage::new(100, 100));
        resized_and_save_image(&img, 50, 50, "resized", "suffix", "png");
        let path = "web/u/resized_suffix.png";
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_image() {
        let img = DynamicImage::ImageRgb8(RgbImage::new(10, 10));
        let result = save_image(&img, "save_test.png");
        assert!(result.is_ok());
        let path = "web/u/save_test.png";
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }
}
