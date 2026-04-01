use super::{Image, ImageFormat};
use crate::shapes::ShapeTrait;
use kurbo::Point;

#[test]
fn test_image_creation() {
    let png_data = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let format = ImageFormat::from_magic_bytes(&png_data);
    assert_eq!(format, Some(ImageFormat::Png));
}

#[test]
fn test_format_detection() {
    assert_eq!(ImageFormat::from_extension("png"), Some(ImageFormat::Png));
    assert_eq!(ImageFormat::from_extension("PNG"), Some(ImageFormat::Png));
    assert_eq!(ImageFormat::from_extension("jpg"), Some(ImageFormat::Jpeg));
    assert_eq!(ImageFormat::from_extension("jpeg"), Some(ImageFormat::Jpeg));
    assert_eq!(ImageFormat::from_extension("webp"), Some(ImageFormat::WebP));
    assert_eq!(ImageFormat::from_extension("gif"), None);
}

#[test]
fn test_fit_within() {
    let data = vec![0u8; 10];
    let img = Image::new(Point::ZERO, &data, 1000, 500, ImageFormat::Png);

    let fitted = img.fit_within(400.0, 400.0);
    assert!((fitted.width - 400.0).abs() < 0.01);
    assert!((fitted.height - 200.0).abs() < 0.01);
}

#[test]
fn test_bounds() {
    let data = vec![0u8; 10];
    let img = Image::new(Point::new(10.0, 20.0), &data, 100, 50, ImageFormat::Png);
    let bounds = img.bounds();
    assert!((bounds.x0 - 10.0).abs() < f64::EPSILON);
    assert!((bounds.y0 - 20.0).abs() < f64::EPSILON);
    assert!((bounds.x1 - 110.0).abs() < f64::EPSILON);
    assert!((bounds.y1 - 70.0).abs() < f64::EPSILON);
}
