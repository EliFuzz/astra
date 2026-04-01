use super::state;
use astra_canvas::canvas::CanvasDocument;

pub fn save_document(document: &CanvasDocument, name: &str) {
    let doc = document.clone();
    let default_name = format!("{}.json", name);
    std::thread::spawn(move || {
        let dialog = rfd::FileDialog::new()
            .set_title("Save Document")
            .set_file_name(&default_name)
            .add_filter("Astra Document", &["json"]);

        if let Some(path) = dialog.save_file() {
            match doc.to_json() {
                Ok(json) => {
                    if let Err(error) = std::fs::write(&path, &json) {
                        log::error!("Failed to write file: {}", error);
                    } else {
                        log::info!("Saved document to: {:?}", path);
                    }
                }
                Err(error) => log::error!("Failed to serialize document: {}", error),
            }
        }
    });
}

pub fn load_document() {
    std::thread::spawn(move || {
        let dialog = rfd::FileDialog::new()
            .set_title("Open Document")
            .add_filter("Astra Document", &["json"])
            .add_filter("Excalidraw", &["excalidraw"]);

        if let Some(path) = dialog.pick_file() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    let is_excalidraw = path
                        .extension()
                        .map(|extension| {
                            extension
                                .to_string_lossy()
                                .eq_ignore_ascii_case("excalidraw")
                        })
                        .unwrap_or(false);

                    let result = if is_excalidraw {
                        CanvasDocument::shapes_from_json(&content).map_err(|error| error.to_string())
                    } else {
                        CanvasDocument::from_json(&content).map_err(|error| error.to_string())
                    };

                    match result {
                        Ok(document) => {
                            log::info!("Loaded document from: {:?}", path);
                            state::set_pending_document(document);
                        }
                        Err(error) => log::error!("Failed to parse document: {}", error),
                    }
                }
                Err(error) => log::error!("Failed to read file: {}", error),
            }
        }
    });
}

pub fn load_document_by_name(_name: &str) {
    load_document()
}
