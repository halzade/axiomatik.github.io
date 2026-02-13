use crate::data::image_validator::{validate_image_width, ImageValidationError};
use image::imageops::Lanczos3;
use image::{DynamicImage, GenericImageView, ImageError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageProcessorError {
    #[error("failed to save image because {0}")]
    ImageFormat(#[from] ImageError),

    #[error("image validation failed {0}")]
    ImageValidationError(#[from] ImageValidationError),
}

pub fn process_images(
    img_data: &[u8],
    file_base: &str,
    ext: &str,
) -> Result<(), ImageProcessorError> {
    let img = image::load_from_memory(img_data)?;
    let (width, height) = img.dimensions();
    validate_image_width(width)?;
    // validate_image_data(&img)?;

    // Save 820xheight
    let img_820 = img.resize(820, height, Lanczos3);
    let name_820 = format!("{}_image_820.{}", file_base, ext);

    // Save 820xany
    save_image(&img_820, name_820.as_str())?;

    // Save 50x50
    resized_and_save_image(&img, 50, 50, file_base, "image_50", ext)?;
    // Save 288x211
    resized_and_save_image(&img, 288, 211, file_base, "image_288", ext)?;
    // Save 440x300
    resized_and_save_image(&img, 440, 300, file_base, "image_440", ext)?;

    Ok(())
}

fn resized_and_save_image(
    img: &DynamicImage,
    w: u32,
    h: u32,
    file_base: &str,
    resolution_image_suffix: &str,
    ext: &str,
) -> Result<(), ImageProcessorError> {
    let resized = img.resize_to_fill(w, h, Lanczos3);
    let name = format!("{}_{}.{}", file_base, resolution_image_suffix, ext);
    save_image(&resized, name.as_str())?;
    Ok(())
}

fn save_image(image: &DynamicImage, file_name: &str) -> Result<(), ImageProcessorError> {
    image.save(format!("web/u/{}", file_name))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust;
    use crate::trust::me::TrustError;
    use image::{DynamicImage, RgbImage};
    use std::path::Path;
    use DynamicImage::ImageRgb8;

    #[test]
    fn test_process_images() -> Result<(), TrustError> {
        let mut img_data = Vec::new();
        ImageRgb8(RgbImage::new(1000, 1000))
            .write_to(&mut std::io::Cursor::new(&mut img_data), image::ImageFormat::Png)?;

        let _result = process_images(&img_data, "test_image", "png")?;

        trust::me::path_exists("web/u/test_image_image_50.png")?;
        trust::me::path_exists("web/u/test_image_image_288.png")?;
        trust::me::path_exists("web/u/test_image_image_288.png")?;
        trust::me::path_exists("web/u/test_image_image_440.png")?;
        trust::me::path_exists("web/u/test_image_image_820.png")?;

        trust::me::remove_file("web/u/test_image_image_50.png")?;
        trust::me::remove_file("web/u/test_image_image_288.png")?;
        trust::me::remove_file("web/u/test_image_image_440.png")?;
        trust::me::remove_file("web/u/test_image_image_820.png")?;

        Ok(())
    }

    #[test]
    fn test_process_images_too_small() {
        let mut img_data = Vec::new();
        ImageRgb8(RgbImage::new(100, 100))
            .write_to(&mut std::io::Cursor::new(&mut img_data), image::ImageFormat::Png)
            .unwrap();
        let result = process_images(&img_data, "test_small", "png");

        assert!(result.is_err());
    }

    #[test]
    fn test_resized_and_save_image() -> Result<(), TrustError> {
        let img = ImageRgb8(RgbImage::new(100, 100));
        let res = resized_and_save_image(&img, 50, 50, "resize_me", "image_50", "png");

        assert!(res.is_ok());
        assert!(Path::new("web/u/resize_me_image_50.png").exists());
        trust::me::remove_file("web/u/resize_me_image_50.png")?;

        Ok(())
    }
}
