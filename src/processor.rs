use crate::validate_media::{validate_data_is_audio, validate_data_is_video, MediaValidationError};
use image::imageops::Lanczos3;
use image::{DynamicImage, GenericImageView};
use std::fs;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown category: {0}")]
    UnknownCategory(String),

    #[error("Something wrong with the image data")]
    InvalidImageFormatError,

    #[error("Media validation error: {0}")]
    MediaValidation(#[from] MediaValidationError),
}

pub fn process_audio(audio_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    validate_data_is_audio(audio_data)?;
    save_audio(audio_data, file_name)
}

pub fn process_video(video_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    validate_data_is_video(video_data)?;
    save_video(video_data, file_name)
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
    image
        .save(format!("web/uploads/{}", file_name))
        .map_err(|e| {
            error!("Failed to save image {}: {}", file_name, e);
            ProcessorError::InvalidImageFormatError
        })
}

pub fn save_video(video_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    fs::write(format!("web/uploads/{}", file_name), video_data).map_err(|e| {
        error!("Failed to save video {}: {}", file_name, e);
        ProcessorError::Io(e)
    })
}

pub fn save_audio(audio_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    fs::write(format!("web/uploads/{}", file_name), audio_data).map_err(|e| {
        error!("Failed to save audio {}: {}", file_name, e);
        ProcessorError::Io(e)
    })
}

pub fn process_category(raw_category: &str) -> Result<String, ProcessorError> {
    match raw_category {
        "zahranici" => Ok("zahraničí".into()),
        "republika" => Ok("republika".into()),
        "finance" => Ok("finance".into()),
        "technologie" => Ok("technologie".into()),
        "veda" => Ok("věda".into()),
        cat => {
            error!("Unknown category: {}", cat);
            Err(ProcessorError::UnknownCategory(cat.to_string()))
        }
    }
}

// TODO
pub fn process_short_text(raw_text: &str) -> String {
    raw_text
        .replace("\r\n", "\n")
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().replace("\n", "<br>\n"))
        .collect::<Vec<String>>()
        .join("</p><p>")
}

// TODO
pub fn process_text(raw_text: &str) -> String {
    raw_text
        .replace("\r\n", "\n")
        .split("\n\n\n")
        .filter(|block| !block.trim().is_empty())
        .map(|block| {
            let inner_html = block
                .split("\n\n")
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    if s.starts_with("   ") {
                        format!("<blockquote>{}</blockquote>", s.trim())
                    } else {
                        format!("<p>{}</p>", s.trim().replace("\n", " "))
                    }
                })
                .collect::<Vec<String>>()
                .join("");
            format!("<div class=\"container\">{}</div>", inner_html)
        })
        .collect::<Vec<String>>()
        .join("")
}
