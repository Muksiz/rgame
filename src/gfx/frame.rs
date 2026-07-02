//! A retro CPU framebuffer, the size of an imaginary handheld screen. Both the
//! Macroquad window and the headless snapshot tool draw the exact same pixels;
//! the window just scales them up.

use crate::gfx::atlas::{Atlas, TILE};

pub const FB_W: usize = 480;
pub const FB_H: usize = 270;

pub struct Frame {
    /// RGBA8, row-major, FB_W × FB_H.
    pub px: Vec<u8>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            px: vec![0; FB_W * FB_H * 4],
        }
    }

    pub fn clear(&mut self, c: (u8, u8, u8)) {
        for i in (0..self.px.len()).step_by(4) {
            self.px[i] = c.0;
            self.px[i + 1] = c.1;
            self.px[i + 2] = c.2;
            self.px[i + 3] = 255;
        }
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, c: (u8, u8, u8)) {
        if x < 0 || y < 0 || x >= FB_W as i32 || y >= FB_H as i32 {
            return;
        }
        let i = (y as usize * FB_W + x as usize) * 4;
        self.px[i] = c.0;
        self.px[i + 1] = c.1;
        self.px[i + 2] = c.2;
        self.px[i + 3] = 255;
    }

    /// Blend a pixel over the buffer with alpha 0..=255.
    #[inline]
    pub fn blend(&mut self, x: i32, y: i32, c: (u8, u8, u8), a: u8) {
        if x < 0 || y < 0 || x >= FB_W as i32 || y >= FB_H as i32 || a == 0 {
            return;
        }
        let i = (y as usize * FB_W + x as usize) * 4;
        let a = a as u16;
        for (k, ch) in [c.0, c.1, c.2].into_iter().enumerate() {
            let old = self.px[i + k] as u16;
            self.px[i + k] = ((ch as u16 * a + old * (255 - a)) / 255) as u8;
        }
        self.px[i + 3] = 255;
    }

    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, c: (u8, u8, u8)) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.set(xx, yy, c);
            }
        }
    }

    pub fn fill_a(&mut self, x: i32, y: i32, w: i32, h: i32, c: (u8, u8, u8), a: u8) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.blend(xx, yy, c, a);
            }
        }
    }

    /// Blit sprite `id` with its top-left at (x, y), lit by `light` (see
    /// [`crate::gfx::shade`]). Transparent atlas pixels are skipped.
    pub fn sprite(&mut self, atlas: &Atlas, id: u16, x: i32, y: i32, light: f32) {
        self.sprite_scaled(atlas, id, x, y, 1, light);
    }

    /// Same, integer-upscaled (for portraits and the title crab).
    pub fn sprite_scaled(
        &mut self,
        atlas: &Atlas,
        id: u16,
        x: i32,
        y: i32,
        scale: i32,
        light: f32,
    ) {
        for sy in 0..TILE {
            for sx in 0..TILE {
                let (r, g, b, a) = atlas.pixel(id, sx, sy);
                if a < 8 {
                    continue;
                }
                let c = crate::gfx::shade((r, g, b), light);
                for dy in 0..scale {
                    for dx in 0..scale {
                        let px = x + sx as i32 * scale + dx;
                        let py = y + sy as i32 * scale + dy;
                        if a == 255 {
                            self.set(px, py, c);
                        } else {
                            self.blend(px, py, c, a);
                        }
                    }
                }
            }
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}
