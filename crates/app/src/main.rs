#[cfg(feature = "native")]
fn main() {
    env_logger::init();
    pollster::block_on(astra_app::App::run());
}

#[cfg(not(feature = "native"))]
fn main() {
    panic!("Native feature not enabled. Use `cargo run --features native`");
}
