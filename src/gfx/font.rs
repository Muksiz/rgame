//! Text on the framebuffer: the classic public-domain 8×8 bitmap font, with a
//! gentle fallback so the prose's typographic flourishes survive the 8×8 grid
//! (— becomes -, ◆ becomes a real drawn diamond elsewhere, etc.).

use font8x8::{BASIC_FONTS, UnicodeFonts};

use crate::gfx::frame::Frame;

pub const GLYPH: i32 = 8;

/// Fold fancy characters down to something the 8×8 font can draw.
fn fold(c: char) -> Option<char> {
    if (c as u32) < 0x80 {
        return Some(c);
    }
    Some(match c {
        '—' | '–' | '−' => '-',
        '‘' | '’' => '\'',
        '“' | '”' => '"',
        '·' | '∙' | '●' | '•' => '.',
        '○' | '◇' => 'o',
        '◆' | '✦' | '✶' | '☘' | '❀' | '✿' => '*',
        '▸' | '➤' | '►' => '>',
        '∕' => '/',
        'é' => 'e',
        'ö' => 'o',
        'ä' => 'a',
        '☀' => 'O',
        '☾' => 'C',
        '…' => '~', // pre-expanded by `fold_str`; a lone one becomes a squiggle
        _ => return None,
    })
}

/// Fold a whole string (expands … to three dots, drops what can't be drawn).
pub fn fold_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c == '…' {
            out.push_str("...");
        } else if let Some(f) = fold(c) {
            out.push(f);
        }
    }
    out
}

pub fn text_width(s: &str, scale: i32) -> i32 {
    fold_str(s).chars().count() as i32 * GLYPH * scale
}

/// Draw `s` with its top-left at (x, y). Returns the x just past the text.
pub fn text(fb: &mut Frame, x: i32, y: i32, s: &str, c: (u8, u8, u8), scale: i32) -> i32 {
    let mut cx = x;
    for ch in fold_str(s).chars() {
        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        for dy in 0..scale {
                            for dx in 0..scale {
                                fb.set(cx + col * scale + dx, y + row as i32 * scale + dy, c);
                            }
                        }
                    }
                }
            }
        }
        cx += GLYPH * scale;
    }
    cx
}

pub fn text_center(fb: &mut Frame, cx: i32, y: i32, s: &str, c: (u8, u8, u8), scale: i32) {
    text(fb, cx - text_width(s, scale) / 2, y, s, c, scale);
}

/// The one-and-a-half-size face used by the HUD bars, menus and dialogue:
/// each 8×8 glyph fills a 12×12 box (every other glyph pixel doubles, the
/// fixed-point way), so the reading text sits comfortably between the tiny
/// scale-1 captions and the shouty scale-2 headlines.
pub const GLYPH_LG: i32 = 12;
/// Line step for `text_lg` blocks — a 12px glyph plus a pixel of air.
pub const LINE_LG: i32 = 13;

pub fn text_width_lg(s: &str) -> i32 {
    fold_str(s).chars().count() as i32 * GLYPH_LG
}

/// Draw `s` at one-and-a-half size, top-left at (x, y). Returns the x just
/// past the text.
pub fn text_lg(fb: &mut Frame, x: i32, y: i32, s: &str, c: (u8, u8, u8)) -> i32 {
    let mut cx = x;
    for ch in fold_str(s).chars() {
        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                let row = row as i32;
                let (y0, y1) = (row * 3 / 2, (row + 1) * 3 / 2);
                for col in 0..8i32 {
                    if bits & (1 << col) != 0 {
                        let (x0, x1) = (col * 3 / 2, (col + 1) * 3 / 2);
                        for yy in y0..y1 {
                            for xx in x0..x1 {
                                fb.set(cx + xx, y + yy, c);
                            }
                        }
                    }
                }
            }
        }
        cx += GLYPH_LG;
    }
    cx
}

pub fn text_center_lg(fb: &mut Frame, cx: i32, y: i32, s: &str, c: (u8, u8, u8)) {
    text_lg(fb, cx - text_width_lg(s) / 2, y, s, c);
}

/// Greedy word-wrap to `cols` columns, honoring explicit newlines.
pub fn wrap(s: &str, cols: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for raw in s.split('\n') {
        if raw.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut line = String::new();
        for word in raw.split_whitespace() {
            let need = if line.is_empty() { 0 } else { 1 } + word.chars().count();
            if !line.is_empty() && line.chars().count() + need > cols {
                lines.push(std::mem::take(&mut line));
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        lines.push(line);
    }
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_respects_width_and_newlines() {
        let lines = wrap("a quiet morning in Emberwick\n\nnew paragraph", 12);
        assert!(lines.iter().all(|l| l.chars().count() <= 12));
        assert!(lines.contains(&String::new()));
    }

    #[test]
    fn folding_keeps_ascii_and_tames_typography_diacritics() {
        assert_eq!(fold_str("cast — c"), "cast - c");
        assert_eq!(fold_str("wait…"), "wait...");
    }
}
