use crate::shapes::SerializableColor;

pub(in crate::canvas::document::export) fn parse_hex_color(color: &str) -> SerializableColor {
    if color == "transparent" {
        return SerializableColor::transparent();
    }

    if let Some(hex) = color.strip_prefix('#') {
        let hex = hex.trim();
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).unwrap_or(0) * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).unwrap_or(0) * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).unwrap_or(0) * 17;
                return SerializableColor::new(r, g, b, 255);
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                return SerializableColor::new(r, g, b, 255);
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
                return SerializableColor::new(r, g, b, a);
            }
            _ => {}
        }
    }

    SerializableColor::black()
}
