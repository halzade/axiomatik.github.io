use image::imageops::Lanczos3;
use image::{DynamicImage, GenericImageView};
use std::fs;
use thiserror::Error;
use tracing::error;
use crate::validation::validate_media::{validate_data_is_audio, validate_data_is_video, MediaValidationError};

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
        .save(format!("web/u/{}", file_name))
        .map_err(|e| {
            error!("Failed to save image {}: {}", file_name, e);
            ProcessorError::InvalidImageFormatError
        })
}

pub fn save_video(video_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    fs::write(format!("web/u/{}", file_name), video_data).map_err(|e| {
        error!("Failed to save video {}: {}", file_name, e);
        ProcessorError::Io(e)
    })
}

pub fn save_audio(audio_data: &[u8], file_name: &str) -> Result<(), ProcessorError> {
    fs::write(format!("web/u/{}", file_name), audio_data).map_err(|e| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbImage};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_process_audio() {
        // MP3 Magic number: ID3
        let mp3_data = [0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let file_name = "test_audio.mp3";
        let result = process_audio(&mp3_data, file_name);
        assert!(result.is_ok());
        let path = format!("web/u/{}", file_name);
        assert!(Path::new(&path).exists());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_process_video() {
        // MP4 Magic number (ftyp)
        let mp4_data = [
            0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d,
        ];
        let file_name = "test_video.mp4";
        let result = process_video(&mp4_data, file_name);
        assert!(result.is_ok());
        let path = format!("web/u/{}", file_name);
        assert!(Path::new(&path).exists());
        fs::remove_file(path).unwrap();
    }

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

    #[test]
    fn test_save_video() {
        let data = b"video data";
        let result = save_video(data, "save_video.mp4");
        assert!(result.is_ok());
        let path = "web/u/save_video.mp4";
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_save_audio() {
        let data = b"audio data";
        let result = save_audio(data, "save_audio.mp3");
        assert!(result.is_ok());
        let path = "web/u/save_audio.mp3";
        assert!(Path::new(path).exists());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_process_category() {
        assert_eq!(process_category("zahranici").unwrap(), "zahraničí");
        assert_eq!(process_category("republika").unwrap(), "republika");
        assert_eq!(process_category("finance").unwrap(), "finance");
        assert_eq!(process_category("technologie").unwrap(), "technologie");
        assert_eq!(process_category("veda").unwrap(), "věda");
        assert!(process_category("invalid").is_err());
    }

    #[test]
    fn test_process_short_text() {
        let input = "Para 1\r\n\r\nPara 2\nLine 2";
        let output = process_short_text(input);
        assert_eq!(output, "Para 1</p><p>Para 2<br>\nLine 2");
    }

    #[test]
    fn test_process_text() {
        let input = "Block 1 Para 1\n\nBlock 1 Para 2\n\n\n   Block 2 Quote\n\nBlock 2 Para";
        let output = process_text(input);
        assert!(output.contains("<div class=\"container\">"));
        assert!(output.contains("<p>Block 1 Para 1</p>"));
        assert!(output.contains("<p>Block 1 Para 2</p>"));
        assert!(output.contains("<blockquote>Block 2 Quote</blockquote>"));
        assert!(output.contains("<p>Block 2 Para</p>"));
    }
}
