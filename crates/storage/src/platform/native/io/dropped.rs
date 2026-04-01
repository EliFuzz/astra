use astra_canvas::canvas::{Canvas, CanvasDocument};
use astra_canvas::shapes::{Image, ImageFormat, Shape};
use image::GenericImageView;
use std::path::Path;

pub enum DroppedFile {
    Document(CanvasDocument),
    Shape(Shape),
}

pub fn load_dropped_file(path: &Path, canvas: &Canvas) -> Result<Option<DroppedFile>, String> {
    let Some(extension) = path.extension() else {
        return Ok(None);
    };

    let extension = extension.to_string_lossy().to_ascii_lowercase();
    if !matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "webp") {
        return Ok(None);
    }

    let data = std::fs::read(path).map_err(|error| error.to_string())?;

    if extension == "png" {
        if let Some(json) = astra_core::png::extract_scene_from_png(&data) {
            let document = CanvasDocument::from_json(&json).map_err(|error| error.to_string())?;
            return Ok(Some(DroppedFile::Document(document)));
        }
    }

    let format = match extension.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        _ => ImageFormat::Png,
    };

    let decoded = image::load_from_memory(&data).map_err(|error| error.to_string())?;
    let (width, height) = decoded.dimensions();
    let center = canvas.camera.viewport_center_world(canvas.viewport_size);
    let image = Image::new_centered(center, &data, width, height, format);
    Ok(Some(DroppedFile::Shape(Shape::Image(image))))
}
