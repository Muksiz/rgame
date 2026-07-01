use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::app::App;
use crate::content::quests::QUESTS;
use crate::ui::{daylight, effects, put_str, shade};
use crate::world::camera;
use crate::world::map::{Tile, hash2};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    if area.height < 5 || area.width < 20 {
        return;
    }
    let top = Rect { height: 1, ..area };
    let map_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: area.height - 2,
    };
    let bottom = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };

    let dl = daylight(app.tick);
    draw_map(frame.buffer_mut(), map_area, app, dl);
    effects::weather(
        frame.buffer_mut(),
        map_area,
        app.zone().weather,
        app.tick,
        dl,
    );
    draw_top_bar(frame.buffer_mut(), top, app, dl);
    draw_bottom_bar(frame.buffer_mut(), bottom, app);
}

fn draw_map(buf: &mut Buffer, area: Rect, app: &App, dl: f32) {
    let (vw, vh) = (area.width as i32, area.height as i32);
    let (ox, oy) = camera::viewport_origin(app.player, vw, vh);
    let zone = app.zone();
    let lantern_lit = zone.id == 0 && app.completed.contains(&1);

    for row in 0..vh {
        for col in 0..vw {
            let (wx, wy) = (ox + col, oy + row);
            let tile = zone.tile(wx, wy);
            let (ch, fg, bg) = tile_visual(tile, wx, wy, app.tick, zone.seed, lantern_lit);
            if let Some(cell) = buf.cell_mut((area.x + col as u16, area.y + row as u16)) {
                cell.set_char(ch);
                cell.set_fg(shade(fg, dl));
                cell.set_bg(shade(bg, dl));
            }
        }
    }

    let to_screen = |wx: i32, wy: i32| -> Option<(u16, u16)> {
        let (sx, sy) = (wx - ox, wy - oy);
        (sx >= 0 && sy >= 0 && sx < vw && sy < vh).then(|| (area.x + sx as u16, area.y + sy as u16))
    };
    let put = |buf: &mut Buffer, wx: i32, wy: i32, ch: char, fg: (u8, u8, u8), bold: bool| {
        if let Some((sx, sy)) = to_screen(wx, wy)
            && let Some(cell) = buf.cell_mut((sx, sy))
        {
            cell.set_char(ch);
            let mut style = Style::new().fg(shade(fg, dl.max(0.55)));
            if bold {
                style = style.bold();
            }
            cell.set_style(style);
        }
    };

    for critter in &zone.critters {
        put(
            buf,
            critter.pos.0,
            critter.pos.1,
            critter.kind.glyph(),
            critter.kind.color(),
            false,
        );
    }

    let active = app.active_quest().map(|q| q.id);
    for npc in &zone.npcs {
        put(buf, npc.pos.0, npc.pos.1, npc.glyph, npc.color, true);
        // Quest markers hover above heads: `!` = has a quest for you,
        // `?` = waiting on your rune.
        if npc.quest == active {
            let accepted = active.map(|id| app.accepted.contains(&id)).unwrap_or(false);
            let bob = (app.tick / 8).is_multiple_of(2);
            if accepted {
                put(buf, npc.pos.0, npc.pos.1 - 1, '?', (130, 210, 230), true);
            } else if bob {
                put(buf, npc.pos.0, npc.pos.1 - 1, '!', (255, 220, 90), true);
            }
        }
    }

    put(buf, app.player.0, app.player.1, '@', (245, 236, 205), true);
}

fn draw_top_bar(buf: &mut Buffer, area: Rect, app: &App, dl: f32) {
    fill_bar(buf, area);
    let zone = app.zone();

    let diamonds: String = QUESTS
        .iter()
        .filter(|q| q.zone == zone.id)
        .map(|q| {
            if app.completed.contains(&q.id) {
                '◆'
            } else {
                '◇'
            }
        })
        .collect();
    let time_icon = if dl > 0.62 {
        "☀"
    } else if dl < 0.35 {
        "☾"
    } else {
        "✦"
    };
    let text = format!(
        " {time_icon}  {}   {diamonds}   runes {}∕{}",
        zone.name,
        app.completed.len(),
        QUESTS.len()
    );
    put_str(
        buf,
        area,
        area.x,
        area.y,
        &text,
        bar_style().fg(Color::Rgb(226, 208, 168)),
    );

    // The status-bar cat, on its endless patrol.
    let cat = if (app.tick / 600).is_multiple_of(5) {
        "=-.-=zZ"
    } else {
        "=^.^="
    };
    let span = area
        .width
        .saturating_sub(text.chars().count() as u16 + cat.len() as u16 + 6) as u64;
    if span > 4 {
        let t = (app.tick / 20) % (span * 2);
        let offset = if t < span { t } else { span * 2 - t };
        let x = area.x + text.chars().count() as u16 + 3 + offset as u16;
        put_str(
            buf,
            area,
            x,
            area.y,
            cat,
            bar_style().fg(Color::Rgb(178, 162, 140)),
        );
    }
}

fn draw_bottom_bar(buf: &mut Buffer, area: Rect, app: &App) {
    fill_bar(buf, area);
    if let Some((msg, _)) = &app.toast {
        put_str(
            buf,
            area,
            area.x + 1,
            area.y,
            msg,
            bar_style().fg(Color::Rgb(255, 224, 150)),
        );
        return;
    }
    let near_npc = app
        .zone()
        .npcs
        .iter()
        .find(|n| (n.pos.0 - app.player.0).abs() <= 1 && (n.pos.1 - app.player.1).abs() <= 1);
    let hint = match near_npc {
        Some(npc) => format!(" e talk to {}  ·  c cast  ·  q journal  ·  f hint  ·  esc rest", npc.name),
        None => " move: ←↓↑→ / wasd / hjkl  ·  e talk  ·  c cast rune  ·  q journal  ·  f Ferris hint  ·  esc rest".to_string(),
    };
    put_str(
        buf,
        area,
        area.x,
        area.y,
        &hint,
        bar_style().fg(Color::Rgb(150, 140, 122)),
    );
}

fn fill_bar(buf: &mut Buffer, area: Rect) {
    for x in area.x..area.x + area.width {
        if let Some(cell) = buf.cell_mut((x, area.y)) {
            cell.set_char(' ');
            cell.set_style(bar_style());
        }
    }
}

fn bar_style() -> Style {
    Style::new().bg(Color::Rgb(22, 19, 14))
}

#[allow(clippy::type_complexity)]
fn tile_visual(
    tile: Tile,
    x: i32,
    y: i32,
    tick: u64,
    seed: u32,
    lantern_lit: bool,
) -> (char, (u8, u8, u8), (u8, u8, u8)) {
    let h = hash2(x, y, seed);
    let grass_bg = (26, 48, 24);
    match tile {
        Tile::Grass => match h % 12 {
            0 => ('\'', (58, 96, 50), grass_bg),
            1 => (',', (52, 88, 46), grass_bg),
            2 => ('.', (48, 82, 44), grass_bg),
            _ => (' ', grass_bg, grass_bg),
        },
        Tile::TallGrass => {
            let sway = (((tick / 12) as i32) + x) % 2 == 0;
            (if sway { '"' } else { '\'' }, (92, 138, 74), grass_bg)
        }
        Tile::Flower => {
            let c = [
                (224, 140, 168),
                (228, 200, 110),
                (230, 228, 220),
                (186, 160, 230),
            ][(h % 4) as usize];
            ('*', c, grass_bg)
        }
        Tile::Tree => {
            let fg = [(36, 110, 58), (44, 122, 64), (30, 98, 52)][(h % 3) as usize];
            (
                if h.is_multiple_of(3) { '♣' } else { '♠' },
                fg,
                (20, 38, 20),
            )
        }
        Tile::Bush => ('•', (72, 132, 66), (24, 44, 22)),
        Tile::Water => {
            let f = ((x + y * 3) as i64 + (tick / 6) as i64).rem_euclid(8);
            let ch = match f {
                0 | 1 => '~',
                2 => '≈',
                _ => ' ',
            };
            (ch, (86, 140, 210), (16, 38, 76))
        }
        Tile::Reed => {
            let sway = (((tick / 14) as i32) + x) % 2 == 0;
            (if sway { '¦' } else { '|' }, (134, 164, 94), (22, 44, 40))
        }
        Tile::Bridge => ('=', (150, 118, 78), (74, 54, 34)),
        Tile::Path => match h % 12 {
            0 => ('·', (128, 108, 78), (92, 76, 52)),
            1 => ('.', (120, 100, 72), (92, 76, 52)),
            _ => (' ', (92, 76, 52), (92, 76, 52)),
        },
        Tile::Sand => match h % 8 {
            0 => ('.', (150, 132, 90), (118, 102, 66)),
            1 => ('~', (142, 124, 84), (118, 102, 66)),
            _ => (' ', (118, 102, 66), (118, 102, 66)),
        },
        Tile::Wall => (
            if h.is_multiple_of(7) { '▪' } else { ' ' },
            (92, 86, 76),
            (64, 58, 50),
        ),
        Tile::Roof => {
            let ch = match h % 9 {
                0 => '/',
                1 => '\\',
                _ => ' ',
            };
            (ch, (134, 80, 54), (104, 58, 40))
        }
        Tile::Door => ('+', (200, 172, 122), (58, 40, 24)),
        Tile::Floor => (' ', (54, 44, 32), (54, 44, 32)),
        Tile::Fence => ('┼', (146, 116, 76), grass_bg),
        Tile::Cliff => {
            let fg = [(96, 94, 108), (106, 104, 118), (88, 86, 100)][(h % 3) as usize];
            (
                if h.is_multiple_of(3) { '^' } else { '▲' },
                fg,
                (44, 44, 52),
            )
        }
        Tile::Rock => ('o', (128, 126, 118), (34, 48, 32)),
        Tile::Campfire => {
            let f = ((tick / 3) % 3) as usize;
            let (ch, fg) = [
                ('▲', (244, 168, 58)),
                ('♦', (240, 120, 50)),
                ('▲', (250, 204, 94)),
            ][f];
            (ch, fg, (36, 30, 20))
        }
        Tile::Lantern => {
            if lantern_lit {
                let g = ((tick / 4) % 3) as usize;
                let fg = [(255, 214, 120), (255, 192, 90), (255, 228, 150)][g];
                ('◆', fg, (66, 54, 26))
            } else {
                ('◆', (120, 120, 132), (30, 34, 30))
            }
        }
        Tile::Gate => ('≡', (152, 122, 80), (40, 36, 26)),
        Tile::Sign => ('§', (206, 192, 148), grass_bg),
    }
}
