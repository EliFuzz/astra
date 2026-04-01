pub const PNG_SCENE_METADATA_KEY: &str = "application/vnd.astra+json";

pub fn encode_png(
    rgba_data: &[u8],
    width: u32,
    height: u32,
    scene_json: Option<&str>,
) -> Option<Vec<u8>> {
    let mut png_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_data, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        if let Some(json) = scene_json {
            if let Err(e) =
                encoder.add_ztxt_chunk(PNG_SCENE_METADATA_KEY.to_string(), json.to_string())
            {
                log::warn!("Failed to add metadata chunk: {:?}", e);
            }
        }

        let mut writer = match encoder.write_header() {
            Ok(w) => w,
            Err(e) => {
                log::error!("Failed to write PNG header: {:?}", e);
                return None;
            }
        };

        if let Err(e) = writer.write_image_data(rgba_data) {
            log::error!("Failed to write PNG data: {:?}", e);
            return None;
        }
    }

    Some(png_data)
}

pub fn extract_scene_from_png(png_data: &[u8]) -> Option<String> {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_data));
    let reader = decoder.read_info().ok()?;

    for chunk in &reader.info().compressed_latin1_text {
        if chunk.keyword == PNG_SCENE_METADATA_KEY {
            return chunk.get_text().ok();
        }
    }
    None
}
