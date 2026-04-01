pub fn normalize_ws_url(server: &str) -> String {
    let server = server.trim();
    if server.starts_with("ws://") || server.starts_with("wss://") {
        if server.ends_with("/ws") {
            server.to_string()
        } else {
            format!("{}/ws", server.trim_end_matches('/'))
        }
    } else {
        format!("ws://{}/ws", server.trim_end_matches('/'))
    }
}

pub fn derive_ws_url(protocol: &str, host: &str) -> String {
    let ws_protocol = if protocol == "https:" { "wss:" } else { "ws:" };
    format!("{}//{}/ws", ws_protocol, host)
}
