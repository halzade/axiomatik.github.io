use image::{DynamicImage, GenericImageView};
use image::imageops::Lanczos3;
use tracing::error;

pub fn process_audio() {
    // TODO
}

pub fn process_video() {
    // TODO
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
    save_image(&img_820, name_820.as_str());

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
    let resized = img.resize_to_fill(w, h, image::imageops::FilterType::Lanczos3);
    let name = format!("{}_{}.{}", base_name, suffix, ext);
    save_image(&resized, name.as_str());
}

pub fn save_image(image: &DynamicImage, file_name: &str) {
    if let Err(e) = image.save(format!("web/uploads/{}", file_name)) {
        error!("Failed to save image {}: {}", file_name, e);
    }
}
