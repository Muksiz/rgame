use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::world::map::{Weather, hash2};

/// Draw the zone's weather on top of the map. Particles change the glyph and
/// foreground only, keeping the terrain's background so they feel *in* the
/// world rather than on it.
pub fn weather(buf: &mut Buffer, area: Rect, kind: Weather, tick: u64, daylight: f32) {
    match kind {
        Weather::Petals => petals(buf, area, tick),
        Weather::Fireflies => fireflies_weather(buf, area, tick, daylight),
        Weather::Rain => rain(buf, area, tick),
        Weather::Mist => mist(buf, area, tick),
    }
}

fn overlay(
    buf: &mut Buffer,
    area: Rect,
    x: i64,
    y: i64,
    ch: char,
    fg: (u8, u8, u8),
    daylight: f32,
) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (area.x as i64 + x, area.y as i64 + y);
    if x >= (area.x + area.width) as i64 || y >= (area.y + area.height) as i64 {
        return;
    }
    if let Some(cell) = buf.cell_mut((x as u16, y as u16)) {
        cell.set_char(ch);
        cell.set_fg(crate::ui::shade(fg, daylight));
    }
}

fn petals(buf: &mut Buffer, area: Rect, tick: u64) {
    let (w, h) = (area.width as i64, area.height as i64);
    if w == 0 || h == 0 {
        return;
    }
    let count = (w * h / 550).max(6);
    for i in 0..count {
        let bx = hash2(i as i32, 1, 0x9E7A) as i64 % w;
        let by = hash2(i as i32, 2, 0x9E7A) as i64 % h;
        let drift = ((tick / 4) as i64 + i * 3) % (w + h);
        let x = (bx + drift) % w;
        let y = (by + drift / 2) % h;
        let ch = if (tick / 8 + i as u64).is_multiple_of(2) {
            '✿'
        } else {
            '·'
        };
        overlay(buf, area, x, y, ch, (232, 168, 190), 1.0);
    }
}

fn fireflies_weather(buf: &mut Buffer, area: Rect, tick: u64, daylight: f32) {
    // Fireflies come out as the light fades; a few scouts fly in daytime too.
    let dim = 1.0 - daylight;
    let (w, h) = (area.width as i64, area.height as i64);
    if w == 0 || h == 0 {
        return;
    }
    let count = ((w * h / 400) as f32 * (0.25 + 0.75 * dim)).max(4.0) as i64;
    for i in 0..count {
        let bx = hash2(i as i32, 3, 0xF1FE) as i64 % w;
        let by = hash2(i as i32, 4, 0xF1FE) as i64 % h;
        let t = tick as f32;
        let x = bx + ((t / 13.0 + i as f32).sin() * 2.5) as i64;
        let y = by + ((t / 17.0 + i as f32 * 2.0).cos() * 1.5) as i64;
        let lit = (tick / 6 + i as u64 * 3) % 11 < 5;
        if lit {
            overlay(buf, area, x, y, '✶', (226, 232, 130), 1.0);
        }
    }
}

pub fn fireflies(buf: &mut Buffer, area: Rect, tick: u64) {
    fireflies_weather(buf, area, tick, 0.0);
}

fn rain(buf: &mut Buffer, area: Rect, tick: u64) {
    let (w, h) = (area.width as i64, area.height as i64);
    if w == 0 || h == 0 {
        return;
    }
    for x in 0..w {
        let col = hash2(x as i32, 0, 0x0A1D);
        // Two layers of drops at different speeds for a bit of depth.
        for (layer, speed, ch) in [(1u32, 2i64, '│'), (2, 3, '╵')] {
            if col.wrapping_mul(layer) % 10 < 3 {
                let offset = (col.wrapping_mul(layer.wrapping_add(7)) % 500) as i64;
                let span = h + 14;
                let y = (tick as i64 * speed + offset) % span;
                if y < h {
                    overlay(buf, area, x, y, ch, (118, 150, 188), 1.0);
                }
            }
        }
    }
}

fn mist(buf: &mut Buffer, area: Rect, tick: u64) {
    let (w, h) = (area.width as i64, area.height as i64);
    for y in 0..h {
        let band = hash2(0, y as i32, 0x717).is_multiple_of(4);
        if !band {
            continue;
        }
        for x in 0..w {
            let drift_x = x + (tick / 6) as i64;
            let v = hash2((drift_x / 7) as i32, y as i32, 0x718);
            if v % 7 < 3 {
                overlay(buf, area, x, y, '░', (152, 156, 172), 1.0);
            }
        }
    }
}
