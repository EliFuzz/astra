use crate::query_string;
use crate::url;

pub struct UrlParams {
    pub room: Option<String>,
    pub server: Option<String>,
}

pub fn get_url_params() -> UrlParams {
    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            return UrlParams {
                room: None,
                server: None,
            };
        }
    };
    let location = window.location();
    let mut room = None;
    let mut server = None;
    if let Ok(search) = location.search() {
        query_string::merge_room_server_from_url_fragment(&mut room, &mut server, &search);
    }
    if let Ok(hash) = location.hash() {
        query_string::merge_room_server_from_url_fragment(&mut room, &mut server, &hash);
    }
    UrlParams { room, server }
}

pub fn get_room_from_url() -> Option<String> {
    get_url_params().room
}

pub fn get_server_url(server_param: Option<&str>) -> Option<String> {
    if let Some(server) = server_param {
        return Some(url::normalize_ws_url(server));
    }
    let window = web_sys::window()?;
    let location = window.location();
    let protocol = location.protocol().ok()?;
    let host = location.host().ok()?;
    Some(url::derive_ws_url(&protocol, &host))
}
