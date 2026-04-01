mod query_string;
pub mod url;
mod platform;

pub use query_string::{merge_room_server_from_url_fragment, parse_room_server_query};

#[cfg(target_arch = "wasm32")]
pub use platform::wasm::file_ops;

#[cfg(target_arch = "wasm32")]
pub use platform::wasm::{UrlParams, get_room_from_url, get_server_url, get_url_params};
