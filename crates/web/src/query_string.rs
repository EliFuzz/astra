pub fn merge_room_server_from_url_fragment(
    room: &mut Option<String>,
    server: &mut Option<String>,
    raw: &str,
) {
    let (r, s) = parse_room_server_query(raw);
    if room.is_none() {
        *room = r;
    }
    if server.is_none() {
        *server = s;
    }
}

pub fn parse_room_server_query(s: &str) -> (Option<String>, Option<String>) {
    let s = s.trim_start_matches(['?', '#']);
    let mut room = None;
    let mut server = None;
    for pair in s.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if value.is_empty() {
                continue;
            }
            match key {
                "room" => room = Some(value.to_string()),
                "server" => server = Some(value.to_string()),
                _ => {}
            }
        }
    }
    (room, server)
}
