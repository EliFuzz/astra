use astra_canvas::shapes::ImageFormat;

pub(super) fn image_format(file_type: &str, file_name: &str) -> ImageFormat {
    let name = file_name.to_lowercase();
    if file_type.contains("jpeg") || name.ends_with(".jpg") || name.ends_with(".jpeg") {
        ImageFormat::Jpeg
    } else if file_type.contains("webp") || name.ends_with(".webp") {
        ImageFormat::WebP
    } else {
        ImageFormat::Png
    }
}
