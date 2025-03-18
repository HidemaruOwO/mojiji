use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

pub fn dynamic_image_to_vec(
    img: &DynamicImage,
    format: ImageFormat,
) -> Result<Vec<u8>, image::ImageError> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    img.write_to(&mut cursor, format)?;
    Ok(buffer)
}
