use crate::data::settings::{TextSize, get_settings};

/*
 * Scale a design-time pixel size
 */
pub fn sv_16() -> f32 {
    16.0 * scale()
}

pub fn sv_20() -> f32 {
    20.0 * scale()
}

pub fn sv(size: f32) -> f32 {
    size * scale()
}

fn scale() -> f32 {
    match get_settings().map(|s| s.text_size).unwrap_or_default() {
        TextSize::Small => 0.85,
        TextSize::Normal => 1.0,
        TextSize::Large => 1.15,
    }
}
