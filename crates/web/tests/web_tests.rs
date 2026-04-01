use astra_web::url::{derive_ws_url, normalize_ws_url};
use astra_web::{merge_room_server_from_url_fragment, parse_room_server_query};

#[test]
fn normalize_ws_url_adds_scheme_and_path() {
    assert_eq!(normalize_ws_url("localhost:8080"), "ws://localhost:8080/ws");
}

#[test]
fn normalize_ws_url_preserves_existing_scheme() {
    assert_eq!(
        normalize_ws_url("wss://example.com"),
        "wss://example.com/ws"
    );
}

#[test]
fn normalize_ws_url_preserves_trailing_ws() {
    assert_eq!(
        normalize_ws_url("ws://example.com/ws"),
        "ws://example.com/ws"
    );
}

#[test]
fn normalize_ws_url_strips_trailing_slash() {
    assert_eq!(
        normalize_ws_url("ws://example.com/"),
        "ws://example.com/ws"
    );
}

#[test]
fn normalize_ws_url_trims_whitespace() {
    assert_eq!(normalize_ws_url("  localhost  "), "ws://localhost/ws");
}

#[test]
fn derive_ws_url_https_to_wss() {
    assert_eq!(
        derive_ws_url("https:", "example.com"),
        "wss://example.com/ws"
    );
}

#[test]
fn derive_ws_url_http_to_ws() {
    assert_eq!(
        derive_ws_url("http:", "localhost:8080"),
        "ws://localhost:8080/ws"
    );
}

#[test]
fn merge_prefers_first_room_and_fills_server_from_later_fragment() {
    let mut room = None;
    let mut server = None;
    merge_room_server_from_url_fragment(&mut room, &mut server, "?room=a");
    merge_room_server_from_url_fragment(&mut room, &mut server, "?server=s&room=z");
    assert_eq!(room, Some("a".into()));
    assert_eq!(server, Some("s".into()));
}

#[test]
fn parse_room_server_query_with_question_prefix() {
    assert_eq!(
        parse_room_server_query("?room=a&server=b"),
        (Some("a".into()), Some("b".into()))
    );
}

#[test]
fn parse_room_server_query_with_hash_prefix() {
    assert_eq!(
        parse_room_server_query("#room=x"),
        (Some("x".into()), None)
    );
}

#[test]
fn parse_room_server_query_skips_empty_values() {
    assert_eq!(
        parse_room_server_query("room=&server=z"),
        (None, Some("z".into()))
    );
}

#[test]
fn parse_room_server_query_ignores_unknown_keys() {
    assert_eq!(
        parse_room_server_query("foo=1&room=r"),
        (Some("r".into()), None)
    );
}
