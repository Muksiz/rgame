//! The renderer: everything draws CPU-side into a 480×270 framebuffer
//! ([`Frame`]) — sprites for the world, an 8×8 bitmap font for words — and
//! the Macroquad shell (`src/main.rs`) just scales it up, pixel-perfect.
//! `examples/snapshot.rs` dumps the same buffer to PNG, which is how these
//! screens are eyeballed without a window.

pub mod atlas;
pub mod font;
pub mod frame;
pub mod scene;

pub use atlas::Atlas;
pub use frame::{FB_H, FB_W, Frame};
pub use scene::render;

/// The zone's fixed daylight applied to a color, kept as plain RGB so
/// sprites and text share it.
pub fn shade(c: (u8, u8, u8), daylight: f32) -> (u8, u8, u8) {
    let bright = 0.45 + 0.55 * daylight;
    let night = 1.0 - daylight;
    (
        (c.0 as f32 * bright).min(255.0) as u8,
        (c.1 as f32 * bright).min(255.0) as u8,
        (c.2 as f32 * (bright + 0.22 * night)).min(255.0) as u8,
    )
}
