use super::state;
use astra_canvas::canvas::Canvas;
use astra_canvas::shapes::{Image, ImageFormat, Shape};
use image::GenericImageView;

pub fn insert_image_async() {
    std::thread::spawn(move || {
        let dialog = rfd::FileDialog::new()
            .set_title("Insert Image")
            .add_filter("Image", &["png", "jpg", "jpeg", "webp"]);

        if let Some(path) = dialog.pick_file() {
            match std::fs::read(&path) {
                Ok(data) => {
                    let format = path
                        .extension()
                        .and_then(|extension| {
                            ImageFormat::from_extension(&extension.to_string_lossy())
                        })
                        .unwrap_or(ImageFormat::Png);

                    match image::load_from_memory(&data) {
                        Ok(decoded) => {
                            let (width, height) = decoded.dimensions();
                            let image = Image::new_centered(
                                kurbo::Point::new(100.0, 100.0),
                                &data,
                                width,
                                height,
                                format,
                            );
                            state::set_pending_image(Shape::Image(image));
                            log::info!("Image loaded for insertion: {}x{}", width, height);
                        }
                        Err(error) => log::error!("Failed to decode image: {}", error),
                    }
                }
                Err(error) => log::error!("Failed to read image file: {}", error),
            }
        }
    });
}

pub fn export_png(png_data: &[u8], name: &str) {
    let data = png_data.to_vec();
    let default_name = format!("{}.png", name);
    std::thread::spawn(move || {
        let dialog = rfd::FileDialog::new()
            .set_title("Export PNG")
            .set_file_name(&default_name)
            .add_filter("PNG Image", &["png"]);

        if let Some(path) = dialog.save_file() {
            if let Err(error) = std::fs::write(&path, &data) {
                log::error!("Failed to write PNG: {}", error);
            } else {
                log::info!("Exported PNG to: {:?}", path);
            }
        }
    });
}

pub fn pick_png_save_path(name: &str) -> Option<std::path::PathBuf> {
    let default_name = format!("{}.png", name);
    rfd::FileDialog::new()
        .set_title("Export PNG")
        .set_file_name(&default_name)
        .add_filter("PNG Image", &["png"])
        .save_file()
}

pub fn copy_png_to_clipboard(png_data: &[u8], width: u32, height: u32) {
    let image_data = arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::Borrowed(png_data),
    };

    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(error) = clipboard.set_image(image_data) {
                log::error!("Failed to copy PNG to clipboard: {}", error);
            } else {
                log::info!("PNG copied to clipboard ({}x{})", width, height);
            }
        }
        Err(error) => log::error!("Failed to access clipboard: {}", error),
    }
}

pub fn paste_image_from_clipboard(canvas: &Canvas) -> Option<Shape> {
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => match clipboard.get_image() {
            Ok(image_data) => {
                let width = image_data.width as u32;
                let height = image_data.height as u32;
                let png_data =
                    match astra_core::png::encode_png(&image_data.bytes, width, height, None) {
                        Some(data) => data,
                        None => {
                            log::error!("Failed to encode clipboard image as PNG");
                            return None;
                        }
                    };

                let center = canvas.camera.viewport_center_world(canvas.viewport_size);
                let image = Image::new_centered(center, &png_data, width, height, ImageFormat::Png);

                log::info!("Pasted image from clipboard: {}x{}", width, height);
                Some(Shape::Image(image))
            }
            Err(_) => None,
        },
        Err(error) => {
            log::error!("Failed to access clipboard: {}", error);
            None
        }
    }
}
