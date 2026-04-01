#[macro_export]
macro_rules! icon {
    ($name:literal) => {
        ::egui::include_image!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/icons/",
            $name
        ))
    };
}
