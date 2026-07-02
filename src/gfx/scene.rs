//! Draws every game screen onto the framebuffer. This is `ui/` re-imagined in
//! sprites: same state, same tone, 16×16 pixels per tile instead of one glyph.

use crate::app::{App, Dialogue, EPILOGUE, Screen};
use crate::checker::{self, Outcome};
use crate::content::quests::{self, FIZZLE_LINES, PASS_LINES, QUESTS};
use crate::gfx::atlas::{self, Atlas, TILE};
use crate::gfx::font::{self, GLYPH};
use crate::gfx::frame::{FB_H, FB_W, Frame};
use crate::gfx::shade;
use crate::ui::daylight;
use crate::world::camera;
use crate::world::entity::CritterKind;
use crate::world::map::{Tile, Weather, hash2};

/// Visible tiles: 480/16 × ceil(270/16).
const VIEW_W: i32 = (FB_W / TILE) as i32;
const VIEW_H: i32 = (FB_H as i32 + TILE as i32 - 1) / TILE as i32;

const WARM: (u8, u8, u8) = (238, 214, 158);
const GOLD: (u8, u8, u8) = (255, 226, 150);
const DIM: (u8, u8, u8) = (150, 140, 120);
const BODY: (u8, u8, u8) = (222, 212, 188);
const FERRIS_ORANGE: (u8, u8, u8) = (222, 120, 80);
const PANEL_BG: (u8, u8, u8) = (28, 24, 18);
const PANEL_BORDER: (u8, u8, u8) = (196, 164, 110);

pub fn render(fb: &mut Frame, atlas: &Atlas, app: &App) {
    match &app.screen {
        Screen::Title { selected } => title(fb, atlas, app, *selected),
        Screen::Epilogue { page } => epilogue(fb, app, *page),
        _ => {
            world(fb, atlas, app);
            match &app.screen {
                Screen::Dialogue(d) => dialogue(fb, atlas, app, d),
                Screen::Journal => journal(fb, app),
                Screen::Casting { .. } => casting(fb, app),
                Screen::CastResult {
                    quest,
                    outcome,
                    scroll,
                } => cast_result(fb, app, *quest, outcome, *scroll),
                Screen::Paused { selected } => paused(fb, *selected),
                _ => {}
            }
        }
    }
}

// ── the overworld ──────────────────────────────────────────────────────────

fn world(fb: &mut Frame, atlas: &Atlas, app: &App) {
    let dl = daylight(app.tick);
    let (ox, oy) = camera::viewport_origin(app.player, VIEW_W, VIEW_H);
    let zone = app.zone();
    let lantern_lit = zone.id == 0 && app.completed.contains(&1);

    for row in 0..VIEW_H {
        for col in 0..VIEW_W {
            let (wx, wy) = (ox + col, oy + row);
            let tile = zone.tile(wx, wy);
            let (px, py) = (col * TILE as i32, row * TILE as i32);
            let (base, overlay) =
                tile_sprites(tile, wx, wy, app.tick, zone.seed, zone.id, lantern_lit);
            fb.sprite(atlas, base, px, py, dl);
            // A few darker specks so long runs of road/sand don't look flat.
            if matches!(tile, Tile::Path | Tile::Sand) {
                let dark = if tile == Tile::Path {
                    (128, 98, 68)
                } else {
                    (166, 146, 100)
                };
                let h = hash2(wx, wy, zone.seed ^ 0x5EC5);
                for k in 0..2 + (h % 2) {
                    let hx = hash2(wx * 3 + k as i32, wy, 0x5EC5);
                    fb.set(
                        px + (hx % 14) as i32 + 1,
                        py + ((hx >> 8) % 14) as i32 + 1,
                        shade(dark, dl),
                    );
                }
            }
            match tile {
                Tile::Water => shoreline(fb, zone, wx, wy, px, py, dl, app.tick),
                Tile::Path => path_rim(fb, zone, wx, wy, px, py, dl),
                Tile::Roof => roof_detail(fb, zone, wx, wy, px, py, dl),
                Tile::Floor => floor_rim(fb, zone, wx, wy, px, py, dl),
                _ => {}
            }
            if let Some(id) = overlay {
                // Tall greenery casts a little pool of shade.
                if matches!(
                    id,
                    atlas::TREE_GREEN
                        | atlas::TREE_ORANGE
                        | atlas::PINE
                        | atlas::BUSH
                        | atlas::BERRY_BUSH
                        | atlas::STUMP
                        | atlas::SIGN
                ) {
                    blob_shadow(fb, px + 2, py + 12, 12, 3);
                }
                fb.sprite(atlas, id, px, py, dl);
            }
        }
    }

    let to_screen = |wx: i32, wy: i32| -> Option<(i32, i32)> {
        let (sx, sy) = (wx - ox, wy - oy);
        (sx >= 0 && sy >= 0 && sx < VIEW_W && sy < VIEW_H)
            .then_some((sx * TILE as i32, sy * TILE as i32))
    };
    let ent_light = dl.max(0.55);

    for critter in &zone.critters {
        if let Some((px, py)) = to_screen(critter.pos.0, critter.pos.1) {
            let id = match critter.kind {
                CritterKind::Chicken => atlas::CHICKEN,
                CritterKind::Sheep => atlas::SHEEP,
                CritterKind::Frog => atlas::FROG,
                CritterKind::Moth => atlas::MOTH,
            };
            blob_shadow(fb, px + 4, py + 13, 8, 2);
            fb.sprite(atlas, id, px, py, ent_light);
        }
    }

    let active = app.active_quest().map(|q| q.id);
    for npc in &zone.npcs {
        let Some((px, py)) = to_screen(npc.pos.0, npc.pos.1) else {
            continue;
        };
        blob_shadow(fb, px + 3, py + 13, 10, 2);
        fb.sprite(atlas, npc_sprite(npc.quest), px, py, ent_light);
        if npc.quest == active {
            let accepted = active.map(|id| app.accepted.contains(&id)).unwrap_or(false);
            let bob = if (app.tick / 8).is_multiple_of(2) {
                0
            } else {
                1
            };
            if accepted {
                font::text(fb, px + 4, py - 10 + bob, "?", (130, 210, 230), 1);
            } else {
                font::text(fb, px + 4, py - 10 + bob, "!", (255, 220, 90), 1);
            }
        }
    }

    if let Some((px, py)) = to_screen(app.player.0, app.player.1) {
        blob_shadow(fb, px + 3, py + 13, 10, 2);
        fb.sprite(atlas, atlas::PLAYER, px, py, ent_light);
    }

    ambient_life(fb, atlas, app, ox, oy, dl);
    weather(fb, zone.weather, app.tick, dl);
    top_bar(fb, app, dl);
    bottom_bar(fb, app);
}

/// Tile → (opaque base sprite, transparent overlay). The one place tile
/// appearance lives, exactly like `tile_visual` in the TUI.
fn tile_sprites(
    tile: Tile,
    x: i32,
    y: i32,
    tick: u64,
    seed: u32,
    zone_id: usize,
    lantern_lit: bool,
) -> (u16, Option<u16>) {
    let h = hash2(x, y, seed);
    match tile {
        Tile::Grass => (
            if h.is_multiple_of(11) {
                atlas::GRASS_ALT
            } else {
                atlas::GRASS
            },
            grass_decor(h, zone_id),
        ),
        Tile::TallGrass => {
            let sway = (((tick / 12) as i32) + x) % 2 == 0;
            (
                atlas::GRASS,
                Some(if sway {
                    atlas::SPROUT
                } else {
                    atlas::SPROUT_ALT
                }),
            )
        }
        Tile::Flower => (
            [
                atlas::FLOWER_ORANGE,
                atlas::FLOWER_WHITE,
                atlas::FLOWER_BLUE,
            ][(h % 3) as usize],
            None,
        ),
        Tile::Tree => {
            let id = match h % 5 {
                0 | 1 => atlas::TREE_GREEN,
                2 | 3 => atlas::PINE,
                _ => atlas::TREE_ORANGE,
            };
            (atlas::GRASS, Some(id))
        }
        Tile::Bush => (
            atlas::GRASS,
            Some(if h.is_multiple_of(4) {
                atlas::BERRY_BUSH
            } else {
                atlas::BUSH
            }),
        ),
        Tile::Water => (water_frame(x, y, tick), None),
        Tile::Reed => {
            // Riverbank reeds stand on the grass beside the water.
            let sway = (((tick / 14) as i32) + x) % 2 == 0;
            let id = if h.is_multiple_of(4) {
                atlas::LILY
            } else if sway {
                atlas::SPROUT_ALT
            } else {
                atlas::SPROUT
            };
            (atlas::GRASS, Some(id))
        }
        Tile::Bridge => (atlas::BRIDGE, None),
        Tile::Path => (
            if h.is_multiple_of(9) {
                atlas::PATH_ALT
            } else {
                atlas::PATH
            },
            None,
        ),
        Tile::Sand => (atlas::SAND, None),
        Tile::Wall => (atlas::WALL, None),
        Tile::Roof => (atlas::ROOF, None),
        Tile::Door => (atlas::WALL, Some(atlas::DOOR)),
        Tile::Floor => (atlas::FLOOR, None),
        Tile::Fence => (atlas::GRASS, Some(atlas::FENCE)),
        Tile::Cliff => (
            atlas::STONE,
            Some(if h.is_multiple_of(4) {
                atlas::ROCK_BROWN
            } else {
                atlas::ROCK_GREY
            }),
        ),
        Tile::Rock => (atlas::GRASS, Some(atlas::ROCK_GREY)),
        Tile::Campfire => {
            let flame = if (tick / 6).is_multiple_of(2) {
                atlas::CAMPFIRE_A
            } else {
                atlas::CAMPFIRE_B
            };
            (atlas::PATH_ALT, Some(flame))
        }
        Tile::Lantern => {
            let id = if !lantern_lit {
                atlas::TORCH_UNLIT
            } else if (tick / 8).is_multiple_of(2) {
                atlas::TORCH_LIT_A
            } else {
                atlas::TORCH_LIT_B
            };
            (atlas::PATH, Some(id))
        }
        Tile::Gate => (atlas::PATH, Some(atlas::GATE)),
        Tile::Sign => (atlas::GRASS, Some(atlas::SIGN)),
    }
}

/// Sprinkle non-blocking décor over plain grass — each zone grows its own mix
/// (flowers near the village, mushrooms in the woods, pebbles by the spire).
fn grass_decor(h: u32, zone_id: usize) -> Option<u16> {
    if h % 5 != 3 {
        return None;
    }
    const MIXES: [[u16; 5]; 4] = [
        [
            atlas::SPROUT_ALT,
            atlas::FLOWER_SMALL_A,
            atlas::FLOWER_SMALL_B,
            atlas::PEBBLE,
            atlas::SPROUT,
        ],
        [
            atlas::MUSHROOM,
            atlas::SPROUT,
            atlas::STUMP,
            atlas::MUSHROOM_TALL,
            atlas::SPROUT_ALT,
        ],
        [
            atlas::SPROUT,
            atlas::FLOWER_SMALL_A,
            atlas::PEBBLE,
            atlas::SPROUT_ALT,
            atlas::FLOWER_SMALL_B,
        ],
        [
            atlas::PEBBLE,
            atlas::MUSHROOM_TALL,
            atlas::SPROUT_ALT,
            atlas::PEBBLE,
            atlas::SPROUT,
        ],
    ];
    Some(MIXES[zone_id.min(3)][(h / 16) as usize % 5])
}

/// A soft pool of shade under anything that stands up out of the grass.
fn blob_shadow(fb: &mut Frame, x: i32, y: i32, w: i32, h: i32) {
    for dy in 0..h {
        let inset = if h > 1 && (dy == 0 || dy == h - 1) {
            2
        } else {
            0
        };
        for dx in inset..w - inset {
            fb.blend(x + dx, y + dy, (12, 20, 10), 60);
        }
    }
}

/// Tiles that read as water surface (no shoreline drawn between them).
fn is_wet(t: Tile) -> bool {
    matches!(t, Tile::Water | Tile::Bridge)
}

/// Lighter shallows along every water edge that touches land, plus the
/// occasional sun glint out in the open water.
#[allow(clippy::too_many_arguments)]
fn shoreline(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
    tick: u64,
) {
    let t = TILE as i32;
    let shallow = shade((150, 202, 226), dl);
    if !is_wet(zone.tile(wx, wy - 1)) {
        fb.fill_a(px, py, t, 2, shallow, 150);
    }
    if !is_wet(zone.tile(wx, wy + 1)) {
        fb.fill_a(px, py + t - 2, t, 2, shallow, 150);
    }
    if !is_wet(zone.tile(wx - 1, wy)) {
        fb.fill_a(px, py, 2, t, shallow, 150);
    }
    if !is_wet(zone.tile(wx + 1, wy)) {
        fb.fill_a(px + t - 2, py, 2, t, shallow, 150);
    }
    let g = hash2(wx, wy, 0x611);
    if (g % 23) as u64 == (tick / 6) % 23 {
        let (gx, gy) = (px + 3 + (g % 10) as i32, py + 3 + ((g >> 5) % 10) as i32);
        let c = shade((214, 238, 250), dl);
        for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            fb.set(gx + dx, gy + dy, c);
        }
    }
}

/// A one-pixel darker rim where the road meets greenery, so paths read as
/// worn into the grass instead of painted on top.
fn path_rim(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
) {
    let t = TILE as i32;
    let grassy = |tile: Tile| {
        matches!(
            tile,
            Tile::Grass | Tile::TallGrass | Tile::Flower | Tile::Tree | Tile::Bush
        )
    };
    let rim = shade((104, 84, 56), dl);
    if grassy(zone.tile(wx, wy - 1)) {
        fb.fill(px, py, t, 1, rim);
    }
    if grassy(zone.tile(wx, wy + 1)) {
        fb.fill(px, py + t - 1, t, 1, rim);
    }
    if grassy(zone.tile(wx - 1, wy)) {
        fb.fill(px, py, 1, t, rim);
    }
    if grassy(zone.tile(wx + 1, wy)) {
        fb.fill(px + t - 1, py, 1, t, rim);
    }
}

/// Interior floors darken where they meet their walls, so rooms feel enclosed
/// even in the top-down cutaway view.
fn floor_rim(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
) {
    let t = TILE as i32;
    let walled = |tile: Tile| matches!(tile, Tile::Wall | Tile::Roof);
    let rim = shade((52, 38, 26), dl);
    if walled(zone.tile(wx, wy - 1)) {
        fb.fill_a(px, py, t, 3, rim, 150);
    }
    if walled(zone.tile(wx, wy + 1)) {
        fb.fill_a(px, py + t - 2, t, 2, rim, 150);
    }
    if walled(zone.tile(wx - 1, wy)) {
        fb.fill_a(px, py, 2, t, rim, 150);
    }
    if walled(zone.tile(wx + 1, wy)) {
        fb.fill_a(px + t - 2, py, 2, t, rim, 150);
    }
}

/// Shingle lines across roof tiles, and a darker eave along the roof's edge,
/// so houses read as buildings instead of orange slabs.
fn roof_detail(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
) {
    let t = TILE as i32;
    for row in [3, 7, 11, 15] {
        fb.fill_a(px, py + row, t, 1, shade((150, 84, 58), dl), 110);
    }
    let eave = shade((118, 62, 42), dl);
    if zone.tile(wx, wy - 1) != Tile::Roof {
        fb.fill(px, py, t, 1, eave);
    }
    if zone.tile(wx, wy + 1) != Tile::Roof {
        fb.fill(px, py + t - 1, t, 1, eave);
    }
    if zone.tile(wx - 1, wy) != Tile::Roof {
        fb.fill(px, py, 1, t, eave);
    }
    if zone.tile(wx + 1, wy) != Tile::Roof {
        fb.fill(px + t - 1, py, 1, t, eave);
    }
}

/// Butterflies bob around fixed spots in the world by day, and now and then a
/// bird crosses the sky. Small lives, big difference.
fn ambient_life(fb: &mut Frame, atlas: &Atlas, app: &App, ox: i32, oy: i32, dl: f32) {
    use crate::world::map::{MAP_H, MAP_W};
    if dl <= 0.45 {
        return; // butterflies and birds are day folk; fireflies own the night
    }
    let t = app.tick as f32;
    for i in 0..120i32 {
        let ax = (hash2(i, 11, 0xB77F) % (MAP_W as u32 * TILE as u32)) as i32;
        let ay = (hash2(i, 12, 0xB77F) % (MAP_H as u32 * TILE as u32)) as i32;
        let x = ax + ((t / 9.0 + i as f32).sin() * 14.0) as i32 - ox * TILE as i32;
        let y = ay + ((t / 6.0 + i as f32 * 2.0).cos() * 8.0) as i32 - oy * TILE as i32;
        if x < -(TILE as i32) || y < -(TILE as i32) || x >= FB_W as i32 || y >= FB_H as i32 {
            continue;
        }
        let id = if (app.tick / 4 + i as u64).is_multiple_of(2) {
            atlas::BUTTERFLY_A
        } else {
            atlas::BUTTERFLY_B
        };
        fb.sprite(atlas, id, x, y, dl);
    }

    let phase = app.tick % 900;
    if phase < 300 {
        let lane = hash2((app.tick / 900) as i32, 13, 0xB1D);
        let p = phase as i32 * 2;
        let x = if lane.is_multiple_of(2) {
            p - TILE as i32
        } else {
            FB_W as i32 - p
        };
        let y = 24 + (lane % 70) as i32 + ((phase as f32 / 9.0).sin() * 4.0) as i32;
        let id = if (app.tick / 3).is_multiple_of(2) {
            atlas::BIRD_A
        } else {
            atlas::BIRD_B
        };
        fb.sprite(atlas, id, x, y, dl);
    }
}

fn water_frame(x: i32, y: i32, tick: u64) -> u16 {
    if ((x + y * 3) as i64 + (tick / 12) as i64).rem_euclid(2) == 0 {
        atlas::WATER_A
    } else {
        atlas::WATER_B
    }
}

fn npc_sprite(quest: Option<u8>) -> u16 {
    match quest {
        Some(id) if (1..=12).contains(&id) => atlas::NPC_1 + (id as u16 - 1),
        _ => atlas::NPC_1,
    }
}

// ── HUD bars ───────────────────────────────────────────────────────────────

fn top_bar(fb: &mut Frame, app: &App, dl: f32) {
    fb.fill_a(0, 0, FB_W as i32, 13, (22, 19, 14), 215);
    daylight_icon(fb, 8, 6, dl);
    let mut x = font::text(fb, 18, 3, app.zone().name, (226, 208, 168), 1) + 8;
    for q in QUESTS.iter().filter(|q| q.zone == app.zone().id) {
        diamond(fb, x + 3, 6, 3, GOLD, app.completed.contains(&q.id));
        x += 10;
    }
    let runes = format!("runes {}/{}", app.completed.len(), QUESTS.len());
    let w = font::text_width(&runes, 1);
    font::text(fb, FB_W as i32 - w - 6, 3, &runes, (178, 162, 140), 1);
}

fn bottom_bar(fb: &mut Frame, app: &App) {
    let (lines, color) = match &app.toast {
        Some((msg, _)) => (font::wrap(msg, 58), (255, 224, 150)),
        None => {
            let near = app.zone().npcs.iter().find(|n| {
                (n.pos.0 - app.player.0).abs() <= 1 && (n.pos.1 - app.player.1).abs() <= 1
            });
            let hint = match near {
                Some(npc) => format!(
                    "e talk to {} . c cast . q journal . f hint . esc rest",
                    npc.name
                ),
                None => "arrows move . e talk . c cast . q journal . f hint . esc".into(),
            };
            (font::wrap(&hint, 58), DIM)
        }
    };
    let lines = &lines[..lines.len().min(3)];
    let bar_h = 5 + lines.len() as i32 * 9;
    fb.fill_a(
        0,
        FB_H as i32 - bar_h,
        FB_W as i32,
        bar_h,
        (22, 19, 14),
        215,
    );
    for (i, line) in lines.iter().enumerate() {
        font::text(
            fb,
            6,
            FB_H as i32 - bar_h + 3 + i as i32 * 9,
            line,
            color,
            1,
        );
    }
}

fn diamond(fb: &mut Frame, cx: i32, cy: i32, r: i32, c: (u8, u8, u8), filled: bool) {
    for dy in -r..=r {
        for dx in -r..=r {
            let d = dx.abs() + dy.abs();
            if d <= r && (filled || d == r) {
                fb.set(cx + dx, cy + dy, c);
            }
        }
    }
}

fn daylight_icon(fb: &mut Frame, cx: i32, cy: i32, dl: f32) {
    if dl > 0.62 {
        for dy in -3..=3i32 {
            for dx in -3..=3i32 {
                if dx * dx + dy * dy <= 9 {
                    fb.set(cx + dx, cy + dy, (250, 214, 100));
                }
            }
        }
    } else if dl < 0.35 {
        for dy in -3..=3i32 {
            for dx in -3..=3i32 {
                let in_moon = dx * dx + dy * dy <= 9;
                let in_bite = (dx - 2) * (dx - 2) + dy * dy <= 6;
                if in_moon && !in_bite {
                    fb.set(cx + dx, cy + dy, (214, 218, 240));
                }
            }
        }
    } else {
        diamond(fb, cx, cy, 3, (230, 226, 190), true);
    }
}

// ── weather ────────────────────────────────────────────────────────────────

/// Pixel-space port of `ui::effects` — particles drift over the world but
/// under the HUD, and never repaint what's beneath them.
fn weather(fb: &mut Frame, kind: Weather, tick: u64, dl: f32) {
    let (w, h) = (FB_W as i64 / 8, FB_H as i64 / 8); // 8px particle grid
    match kind {
        Weather::Petals => {
            for i in 0..26i64 {
                let bx = hash2(i as i32, 1, 0x9E7A) as i64 % w;
                let by = hash2(i as i32, 2, 0x9E7A) as i64 % h;
                let drift = ((tick / 4) as i64 + i * 3) % (w + h);
                let x = ((bx + drift) % w) * 8;
                let y = ((by + drift / 2) % h) * 8;
                let c = shade((232, 168, 190), dl.max(0.5));
                fb.fill(x as i32, y as i32, 2, 2, c);
                fb.set(
                    x as i32 + 2,
                    y as i32 + 1,
                    shade((200, 120, 150), dl.max(0.5)),
                );
            }
        }
        Weather::Fireflies => fireflies(fb, tick, dl),
        Weather::Rain => {
            for gx in 0..(FB_W as i64 / 4) {
                let col = hash2(gx as i32, 0, 0x0A1D);
                for (layer, speed) in [(1u32, 5i64), (2, 8)] {
                    if col.wrapping_mul(layer) % 10 < 3 {
                        let offset = (col.wrapping_mul(layer.wrapping_add(7)) % 500) as i64;
                        let span = FB_H as i64 + 40;
                        let y = (tick as i64 * speed + offset) % span;
                        for dy in 0..7 {
                            fb.blend(gx as i32 * 4, (y + dy) as i32, (118, 150, 188), 170);
                        }
                    }
                }
            }
        }
        Weather::Mist => {
            // Soft horizontal haze streaks drifting sideways.
            for gy in 0..h {
                if !hash2(0, gy as i32, 0x717).is_multiple_of(4) {
                    continue;
                }
                for gx in 0..w {
                    let drift_x = gx + (tick / 6) as i64;
                    if hash2((drift_x / 3) as i32, gy as i32, 0x718) % 7 < 3 {
                        let y = gy as i32 * 8 + (hash2(gx as i32, gy as i32, 0x719) % 4) as i32;
                        fb.fill_a(gx as i32 * 8 - 2, y, 12, 2, (170, 174, 190), 44);
                        fb.fill_a(gx as i32 * 8, y + 2, 8, 1, (170, 174, 190), 28);
                    }
                }
            }
        }
    }
}

/// Fireflies drift over dark screens; brighter the darker it gets.
fn fireflies(fb: &mut Frame, tick: u64, dl: f32) {
    let dim = 1.0 - dl;
    let (w, h) = (FB_W as i64 / 8, FB_H as i64 / 8);
    let count = (30.0 * (0.25 + 0.75 * dim)).max(6.0) as i64;
    for i in 0..count {
        let bx = hash2(i as i32, 3, 0xF1FE) as i64 % w;
        let by = hash2(i as i32, 4, 0xF1FE) as i64 % h;
        let t = tick as f32;
        let x = ((bx + ((t / 13.0 + i as f32).sin() * 2.5) as i64).rem_euclid(w) * 8) as i32;
        let y = ((by + ((t / 17.0 + i as f32 * 2.0).cos() * 1.5) as i64).rem_euclid(h) * 8) as i32;
        if (tick / 6 + i as u64 * 3) % 11 < 5 {
            fb.fill_a(x - 2, y - 2, 6, 6, (226, 232, 130), 36);
            fb.fill(x, y, 2, 2, (240, 244, 160));
        }
    }
}

// ── panels & overlays ──────────────────────────────────────────────────────

/// The cozy bordered panel; returns (x, y, w, h) of the inner area.
fn panel(fb: &mut Frame, x: i32, y: i32, w: i32, h: i32, title: &str) -> (i32, i32, i32, i32) {
    fb.fill_a(0, 0, FB_W as i32, FB_H as i32, (8, 6, 4), 90);
    fb.fill(x, y, w, h, PANEL_BG);
    fb.fill(x, y, w, 2, PANEL_BORDER);
    fb.fill(x, y + h - 2, w, 2, PANEL_BORDER);
    fb.fill(x, y, 2, h, PANEL_BORDER);
    fb.fill(x + w - 2, y, 2, h, PANEL_BORDER);
    // notch the corners so the box reads as rounded
    for &(cx, cy) in &[
        (x, y),
        (x + w - 2, y),
        (x, y + h - 2),
        (x + w - 2, y + h - 2),
    ] {
        fb.fill(cx, cy, 2, 2, PANEL_BG);
    }
    if !title.is_empty() {
        let tw = font::text_width(title, 1);
        fb.fill(x + 8, y - 3, tw + 8, 13, PANEL_BG);
        font::text(fb, x + 12, y, title, WARM, 1);
    }
    (x + 6, y + 8, w - 12, h - 14)
}

fn centered_panel(fb: &mut Frame, w: i32, h: i32, title: &str) -> (i32, i32, i32, i32) {
    panel(
        fb,
        (FB_W as i32 - w) / 2,
        (FB_H as i32 - h) / 2,
        w,
        h,
        title,
    )
}

fn draw_lines(fb: &mut Frame, x: i32, y: i32, lines: &[(String, (u8, u8, u8))]) -> i32 {
    let mut cy = y;
    for (line, color) in lines {
        font::text(fb, x, cy, line, *color, 1);
        cy += 9;
    }
    cy
}

// ── dialogue ───────────────────────────────────────────────────────────────

fn dialogue(fb: &mut Frame, atlas: &Atlas, app: &App, d: &Dialogue) {
    let (w, h) = (440, 100);
    let x = (FB_W as i32 - w) / 2;
    let y = FB_H as i32 - h - 10;
    let (ix, iy, iw, ih) = panel(fb, x, y, w, h, &d.speaker);

    // Portrait: the speaker's sprite, nice and big.
    let portrait = if d.speaker == "Signpost" {
        Some(atlas::SIGN)
    } else {
        app.zone()
            .npcs
            .iter()
            .find(|n| n.name == d.speaker)
            .map(|n| npc_sprite(n.quest))
    };
    let text_x = if let Some(id) = portrait {
        fb.sprite_scaled(atlas, id, ix + 2, iy + (ih - 64) / 2, 4, 1.0);
        ix + 74
    } else {
        ix + 4
    };

    let page = &d.pages[d.page.min(d.pages.len() - 1)];
    let shown: String = page.chars().take(d.revealed).collect();
    let cols = ((ix + iw - text_x) / GLYPH - 1) as usize;
    let lines: Vec<_> = font::wrap(&shown, cols)
        .into_iter()
        .map(|l| (l, BODY))
        .collect();
    draw_lines(fb, text_x, iy + 2, &lines[..lines.len().min(8)]);

    // Page dots + advance arrow.
    let done = d.revealed >= page.chars().count();
    let mut fx = ix + iw - 10 - d.pages.len() as i32 * 8;
    for i in 0..d.pages.len() {
        diamond(fb, fx, iy + ih - 4, 2, DIM, i == d.page);
        fx += 8;
    }
    let arrow = if !done {
        "..."
    } else if d.page + 1 < d.pages.len() {
        ">"
    } else {
        "*"
    };
    font::text(fb, ix + iw - 8, iy + ih - 8, arrow, DIM, 1);
}

// ── journal ────────────────────────────────────────────────────────────────

fn journal(fb: &mut Frame, app: &App) {
    let (ix, iy, iw, _ih) = centered_panel(fb, 420, 210, "Journal");
    let cols = (iw / GLYPH - 1) as usize;
    let mut lines: Vec<(String, (u8, u8, u8))> = Vec::new();
    let push = |s: &str, c: (u8, u8, u8), lines: &mut Vec<(String, (u8, u8, u8))>| {
        for l in font::wrap(s, cols) {
            lines.push((l, c));
        }
    };

    match app.active_quest() {
        None => {
            push("The journey is complete.", GOLD, &mut lines);
            lines.push((String::new(), DIM));
            push(
                "Every rune cast, every quest done. The road stays open for wandering, and the Library has excellent armchairs.",
                BODY,
                &mut lines,
            );
        }
        Some(quest) => {
            push(
                &format!("Quest {} - {}  ({})", quest.id, quest.title, quest.lesson),
                GOLD,
                &mut lines,
            );
            push(
                &format!("{} . {}", quest.npc, app.zones[quest.zone].name),
                DIM,
                &mut lines,
            );
            lines.push((String::new(), DIM));
            if app.accepted.contains(&quest.id) {
                push(quest.reminder, BODY, &mut lines);
                lines.push((String::new(), DIM));
                push(
                    &format!(
                        "Your scroll: {}/{} - edit it, then press c in the game.",
                        checker::QUEST_DIR,
                        quest.file_name
                    ),
                    (150, 200, 160),
                    &mut lines,
                );
                lines.push((String::new(), DIM));
                let revealed = app.hints.get(&quest.id).copied().unwrap_or(0);
                if revealed == 0 {
                    push(
                        "Ferris is napping in your satchel. (f - ask for a hint)",
                        DIM,
                        &mut lines,
                    );
                } else {
                    push("Ferris's hints:", WARM, &mut lines);
                    for hint in quest.hints.iter().take(revealed) {
                        push(&format!("* {hint}"), BODY, &mut lines);
                    }
                    if revealed < quest.hints.len() {
                        push(
                            &format!("({} more - press f)", quest.hints.len() - revealed),
                            DIM,
                            &mut lines,
                        );
                    }
                }
            } else {
                push(
                    &format!(
                        "Someone needs you: find {} in {}. (They'll be under a bobbing '!')",
                        quest.npc, app.zones[quest.zone].name
                    ),
                    BODY,
                    &mut lines,
                );
            }
        }
    }

    lines.push((String::new(), DIM));
    lines.push((
        format!("Runes mastered: {}/12    esc close", app.completed.len()),
        DIM,
    ));
    let max = 20usize;
    draw_lines(fb, ix + 4, iy + 2, &lines[..lines.len().min(max)]);
}

// ── casting & results ──────────────────────────────────────────────────────

fn casting(fb: &mut Frame, app: &App) {
    let (ix, iy, iw, _) = centered_panel(fb, 280, 72, "Casting");
    let spin = ['|', '/', '-', '\\'][(app.tick / 2) as usize % 4];
    let phrase = crate::ui::cast::WEAVING[(app.tick / 24) as usize % 4];
    font::text_center(
        fb,
        ix + iw / 2,
        iy + 8,
        &format!("{spin}  {phrase}  {spin}"),
        WARM,
        1,
    );
    font::text_center(
        fb,
        ix + iw / 2,
        iy + 26,
        "(rustc is reading your scroll)",
        DIM,
        1,
    );
}

fn cast_result(fb: &mut Frame, app: &App, quest_id: u8, outcome: &Outcome, scroll: u16) {
    match outcome {
        Outcome::Pass { .. } => pass_panel(fb, app, quest_id),
        Outcome::CompileFail { stderr } => fizzle_panel(
            fb,
            app,
            "The rune fizzles - no harm done",
            "The compiler couldn't accept the scroll yet. Its note, in full:",
            stderr,
            scroll,
        ),
        Outcome::TestFail { output } => fizzle_panel(
            fb,
            app,
            "So close! The rune sparks, but won't hold",
            "It compiles! But the quest's own judgement found something off:",
            output,
            scroll,
        ),
        Outcome::Timeout => fizzle_panel(
            fb,
            app,
            "The rune spun in circles (we stopped it gently)",
            "It ran for ten whole seconds without finishing - usually a loop that never meets its end.",
            "(no output - the spell was still going in circles when Ferris unplugged it)",
            scroll,
        ),
        Outcome::Error { msg } => fizzle_panel(
            fb,
            app,
            "The satchel snags",
            "Something outside your code went wrong:",
            msg,
            scroll,
        ),
    }
}

fn pass_panel(fb: &mut Frame, app: &App, quest_id: u8) {
    let (ix, iy, iw, ih) = centered_panel(fb, 340, 130, "The rune takes hold!");
    for i in 0..24 {
        let sx = ix + (hash2(i, 5, 0x51AB) % iw as u32) as i32;
        let sy = iy + (hash2(i, 6, 0x51AB) % 30) as i32;
        let glow = (app.tick / 4 + i as u64) % 3;
        let c = [(255, 226, 150), (240, 196, 110), (255, 240, 190)][glow as usize];
        diamond(fb, sx, sy, if i % 3 == 0 { 2 } else { 1 }, c, true);
    }
    let quest = quests::quest(quest_id);
    font::text_center(
        fb,
        ix + iw / 2,
        iy + 42,
        &format!("Quest complete: {}", quest.title),
        (170, 220, 150),
        1,
    );
    let ferris = PASS_LINES[quest_id as usize % PASS_LINES.len()];
    let cols = (iw / GLYPH - 1) as usize;
    let lines: Vec<_> = font::wrap(ferris, cols)
        .into_iter()
        .map(|l| (l, FERRIS_ORANGE))
        .collect();
    draw_lines(fb, ix + 4, iy + 58, &lines);
    font::text(fb, ix + iw - 60, iy + ih - 8, "enter >", DIM, 1);
}

fn fizzle_panel(fb: &mut Frame, app: &App, title: &str, lead: &str, output: &str, scroll: u16) {
    let (ix, iy, iw, ih) = centered_panel(fb, 460, 244, title);
    let cols = (iw / GLYPH - 1) as usize;
    let ferris_line =
        FIZZLE_LINES[hash2(app.tick as i32 / 600, 3, 9) as usize % FIZZLE_LINES.len()];

    let mut head: Vec<(String, (u8, u8, u8))> = Vec::new();
    for l in font::wrap(ferris_line, cols) {
        head.push((l, FERRIS_ORANGE));
    }
    for l in font::wrap(lead, cols) {
        head.push((l, DIM));
    }
    let body_y = draw_lines(fb, ix + 2, iy + 2, &head) + 4;

    let rows = ((iy + ih - 12 - body_y) / 9).max(0) as usize;
    let all: Vec<String> = output.lines().flat_map(|l| font::wrap(l, cols)).collect();
    let start = (scroll as usize).min(all.len().saturating_sub(1));
    let lines: Vec<_> = all[start..(start + rows).min(all.len())]
        .iter()
        .map(|l| (l.clone(), (200, 196, 186)))
        .collect();
    draw_lines(fb, ix + 2, body_y, &lines);
    let footer = "up/down scroll . enter back to the road";
    let w = font::text_width(footer, 1);
    font::text(fb, ix + iw - w - 2, iy + ih - 8, footer, DIM, 1);
}

// ── rest, credits, title ───────────────────────────────────────────────────

fn paused(fb: &mut Frame, selected: usize) {
    let (ix, iy, _iw, _) = centered_panel(fb, 220, 76, "A moment's rest");
    for (i, label) in ["Back to the road", "Save & sleep (quit)"]
        .iter()
        .enumerate()
    {
        let on = i == selected;
        let c = if on { GOLD } else { DIM };
        let marker = if on { "> " } else { "  " };
        font::text(
            fb,
            ix + 10,
            iy + 10 + i as i32 * 14,
            &format!("{marker}{label}"),
            c,
            1,
        );
    }
}

fn epilogue(fb: &mut Frame, app: &App, page: usize) {
    fb.clear((14, 12, 20));
    fireflies(fb, app.tick, 0.0);
    let (ix, iy, iw, ih) = centered_panel(fb, 430, 200, "~ The Great Library ~");
    let text = EPILOGUE[page.min(EPILOGUE.len() - 1)];
    let cols = (iw / GLYPH - 1) as usize;
    let lines: Vec<_> = font::wrap(text, cols)
        .into_iter()
        .map(|l| (l, (226, 214, 186)))
        .collect();
    draw_lines(fb, ix + 4, iy + 4, &lines);
    let footer = format!("enter >  ({}/{})", page + 1, EPILOGUE.len());
    let w = font::text_width(&footer, 1);
    font::text(fb, ix + iw - w - 4, iy + ih - 8, &footer, DIM, 1);
}

fn title(fb: &mut Frame, atlas: &Atlas, app: &App, selected: usize) {
    fb.clear((13, 15, 22));
    fireflies(fb, app.tick, 0.0);

    let cx = FB_W as i32 / 2;
    font::text_center(fb, cx + 2, 40 + 2, "RUNE & ROAD", (60, 45, 25), 3);
    font::text_center(fb, cx, 40, "RUNE & ROAD", (240, 205, 120), 3);
    font::text_center(
        fb,
        cx,
        74,
        "a cosy little journey through the Rust programming language",
        (150, 150, 170),
        1,
    );

    fb.sprite_scaled(atlas, atlas::FERRIS, cx - 150, 120, 5, 1.0);

    let items: Vec<&str> = if app.has_save {
        vec!["Continue the journey", "A new journey", "Quit"]
    } else {
        vec!["A new journey", "Quit"]
    };
    let mut y = 120;
    for (i, label) in items.iter().enumerate() {
        let on = i == selected;
        let c = if on { GOLD } else { (140, 132, 118) };
        let marker = if on { "> " } else { "  " };
        font::text(fb, cx - 60, y, &format!("{marker}{label}"), c, 1);
        y += 18;
    }

    font::text_center(
        fb,
        cx,
        FB_H as i32 - 20,
        "up/down choose . enter set off . keep a code editor handy",
        (110, 105, 95),
        1,
    );
}
