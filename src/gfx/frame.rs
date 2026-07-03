//! A retro CPU framebuffer, the size of an imaginary handheld screen. Both the
//! Macroquad window and the headless snapshot tool draw the exact same pixels;
//! the window just scales them up.

use crate::gfx::atlas::{Atlas, TILE};

/// The game's native logical resolution — a 16:9 handheld screen. Wider
/// displays (ultrawide, superultrawide) render a *taller/wider* framebuffer
/// than this so the picture fills the window edge-to-edge with no black bars;
/// these stay the sensible default (and what the headless tools render at).
pub const FB_W: usize = 480;
pub const FB_H: usize = 270;

pub struct Frame {
    /// RGBA8, row-major, `w` × `h`.
    pub px: Vec<u8>,
    /// The framebuffer's actual size this frame — usually [`FB_W`]×[`FB_H`],
    /// but stretched to match the window's aspect on wide screens.
    pub w: usize,
    pub h: usize,
}

impl Frame {
    pub fn new() -> Self {
        Self::with_size(FB_W, FB_H)
    }

    /// A framebuffer of a given size. `w`/`h` are clamped to at least the
    /// native resolution so the HUD and panels always have room to lay out.
    pub fn with_size(w: usize, h: usize) -> Self {
        let (w, h) = (w.max(FB_W), h.max(FB_H));
        Self {
            px: vec![0; w * h * 4],
            w,
            h,
        }
    }

    /// Resize in place, reusing the buffer. Cheap no-op when unchanged.
    pub fn resize(&mut self, w: usize, h: usize) {
        let (w, h) = (w.max(FB_W), h.max(FB_H));
        if w == self.w && h == self.h {
            return;
        }
        self.w = w;
        self.h = h;
        self.px.resize(w * h * 4, 0);
    }

    /// A bare offscreen buffer at exactly the requested size — no HUD-minimum
    /// clamp, because nothing lays text out here. The world layer draws into
    /// one of these before being zoomed up onto the screen.
    pub fn scratch(w: usize, h: usize) -> Self {
        Self {
            px: vec![0; w * h * 4],
            w,
            h,
        }
    }

    /// Nearest-neighbor upscale of `src` into this frame's top-left corner,
    /// each source pixel becoming a `scale`×`scale` block (clipped at the
    /// edges when the sizes don't divide evenly).
    pub fn blit_scaled(&mut self, src: &Frame, scale: usize) {
        for sy in 0..src.h {
            for ry in 0..scale {
                let dy = sy * scale + ry;
                if dy >= self.h {
                    break;
                }
                for sx in 0..src.w {
                    let si = (sy * src.w + sx) * 4;
                    for rx in 0..scale {
                        let dx = sx * scale + rx;
                        if dx >= self.w {
                            break;
                        }
                        let di = (dy * self.w + dx) * 4;
                        let (r, g, b, a) =
                            (src.px[si], src.px[si + 1], src.px[si + 2], src.px[si + 3]);
                        self.px[di] = r;
                        self.px[di + 1] = g;
                        self.px[di + 2] = b;
                        self.px[di + 3] = a;
                    }
                }
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.w as i32
    }

    pub fn height(&self) -> i32 {
        self.h as i32
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
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return;
        }
        let i = (y as usize * self.w + x as usize) * 4;
        self.px[i] = c.0;
        self.px[i + 1] = c.1;
        self.px[i + 2] = c.2;
        self.px[i + 3] = 255;
    }

    /// Blend a pixel over the buffer with alpha 0..=255.
    #[inline]
    pub fn blend(&mut self, x: i32, y: i32, c: (u8, u8, u8), a: u8) {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 || a == 0 {
            return;
        }
        let i = (y as usize * self.w + x as usize) * 4;
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
