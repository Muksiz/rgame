//! Draws every game screen onto the framebuffer: sprites for the world, the
//! 8×8 bitmap font for words, 16×16 pixels per tile.

use crate::app::{App, DayPhase, Dialogue, DialogueKind, EPILOGUE, EncounterPhase, Screen};
use crate::checker::{self, Outcome};
use crate::content::quests::{self, FIZZLE_LINES, PASS_LINES, QUESTS};
use crate::content::{items, lore, sides, stones, wilds};
use crate::gfx::atlas::{self, Atlas, TILE};
use crate::gfx::font::{self, GLYPH};
use crate::gfx::frame::Frame;
use crate::gfx::shade;
use crate::world::camera;
use crate::world::entity::{CritterKind, Npc};
use crate::world::map::{Tile, Weather, Zone, hash2};
use crate::world::zones::{
    BAKERY, CARPENTER_HOUSE, ECHO_CAVE, GREAT_LIBRARY, SORREL_COTTAGE, STOREHOUSE,
    STOREHOUSE_CELLAR, TILLY_COTTAGE, WOODS_LODGE, WOODS_RUIN,
};

/// How many whole tiles it takes to cover a framebuffer of size `px`. The
/// view grows with the window, so ultrawide screens simply see more world.
fn tiles_across(px: i32) -> i32 {
    (px + TILE as i32 - 1) / TILE as i32
}

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
        Screen::CharSelect { idx, name } => char_select(fb, atlas, app, *idx, name),
        Screen::Epilogue { page } => epilogue(fb, app, *page),
        Screen::Resting { lore, t, wake } => resting(fb, app, *lore, *t, *wake),
        _ => {
            world(fb, atlas, app);
            match &app.screen {
                Screen::Dialogue(d) => dialogue(fb, atlas, app, d),
                Screen::Journal => journal(fb, app),
                Screen::Encounter {
                    rune,
                    selected,
                    phase,
                } => encounter(fb, *rune, *selected, *phase),
                Screen::Grimoire => grimoire(fb, app),
                Screen::Casting { .. } => casting(fb, app),
                Screen::CastResult {
                    quest,
                    outcome,
                    scroll,
                } => cast_result(fb, app, *quest, outcome, *scroll),
                Screen::Paused { selected } => paused(fb, app, *selected),
                _ => {}
            }
        }
    }
}

// ── the overworld ──────────────────────────────────────────────────────────

/// How far the camera leans in: the world layer renders at native 16px tiles
/// into a framebuffer this many times smaller, then upscales — so the world
/// fills the screen with 2× chunkier pixels while the HUD stays fine-grained
/// (and the window-fitting integer scale in main.rs never knows).
const WORLD_ZOOM: usize = 2;

fn world(fb: &mut Frame, atlas: &Atlas, app: &App) {
    let mut wfb = Frame::scratch(fb.w.div_ceil(WORLD_ZOOM), fb.h.div_ceil(WORLD_ZOOM));
    world_scene(&mut wfb, atlas, app);
    fb.blit_scaled(&wfb, WORLD_ZOOM);
    top_bar(fb, app, app.daylight());
    bottom_bar(fb, app);
    zone_banner(fb, app);
}

/// Everything that lives *in* the world — tiles, folk, weather — drawn at
/// native tile scale into whatever frame it's given (see `WORLD_ZOOM`).
fn world_scene(fb: &mut Frame, atlas: &Atlas, app: &App) {
    let dl = app.daylight();
    let t = TILE as i32;
    // The step-glide: the player's *drawn* position slides from the square
    // they left to the one they stand on, and the camera follows it in
    // pixels — so walking scrolls the world smoothly instead of snapping a
    // tile at a time.
    let (glide_x, glide_y) = player_glide(app);
    let (ppx, ppy) = (app.player.0 * t + glide_x, app.player.1 * t + glide_y);
    let (cam_x, cam_y) =
        camera::viewport_origin_px((ppx + t / 2, ppy + t / 2), fb.width(), fb.height());
    let (ox, oy) = (cam_x.div_euclid(t), cam_y.div_euclid(t));
    let (sub_x, sub_y) = (cam_x.rem_euclid(t), cam_y.rem_euclid(t));
    let (view_w, view_h) = (
        tiles_across(fb.width() + sub_x),
        tiles_across(fb.height() + sub_y),
    );
    let zone = app.zone();
    // Emberwick's festival lantern waits for quest 1; every other lantern
    // (the Library's lamps) burns on its own.
    let lantern_lit = zone.id != 0 || app.completed.contains(&1);

    for row in 0..view_h {
        for col in 0..view_w {
            let (wx, wy) = (ox + col, oy + row);
            let tile = zone.tile(wx, wy);
            let (px, py) = (col * TILE as i32 - sub_x, row * TILE as i32 - sub_y);
            let (base, overlay) = tile_sprites(tile, wx, wy, app.tick, zone, lantern_lit);
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
                Tile::Water | Tile::WaterRock => shoreline(fb, zone, wx, wy, px, py, dl, app.tick),
                Tile::Path => {
                    path_rim(fb, zone, wx, wy, px, py, dl, (104, 84, 56));
                    building_shadow(fb, zone, wx, wy, px, py, dl);
                }
                Tile::Plaza => {
                    path_rim(fb, zone, wx, wy, px, py, dl, (96, 94, 88));
                    building_shadow(fb, zone, wx, wy, px, py, dl);
                }
                Tile::Cliff => path_rim(fb, zone, wx, wy, px, py, dl, (88, 92, 88)),
                Tile::Roof => {
                    roof_detail(fb, zone, wx, wy, px, py, dl, (150, 84, 58), (118, 62, 42));
                    roof_peak(fb, zone, wx, wy, px, py, dl, (150, 84, 58));
                }
                Tile::RoofSlate => {
                    roof_detail(fb, zone, wx, wy, px, py, dl, (94, 104, 118), (62, 70, 82));
                    roof_peak(fb, zone, wx, wy, px, py, dl, (94, 104, 118));
                }
                Tile::RoofCream => {
                    roof_detail(
                        fb,
                        zone,
                        wx,
                        wy,
                        px,
                        py,
                        dl,
                        (176, 148, 108),
                        (142, 116, 82),
                    );
                    roof_peak(fb, zone, wx, wy, px, py, dl, (176, 148, 108));
                }
                Tile::Grass | Tile::Sand => building_shadow(fb, zone, wx, wy, px, py, dl),
                Tile::Floor => {
                    floor_rim(fb, zone, wx, wy, px, py, dl);
                    sunbeam(fb, zone, wx, wy, px, py, dl, app.tick);
                }
                Tile::Rug => rug_detail(fb, zone, wx, wy, px, py, dl),
                Tile::Runestone => stone_glimmer(fb, app, wx, wy, px, py),
                Tile::Campfire => smoke_wisp(fb, px, py, app.tick, zone.seed ^ 0x5F0, dl),
                _ => {}
            }
            if let Some(id) = overlay {
                // Tall greenery casts a little pool of shade.
                if matches!(
                    id,
                    atlas::TREE_GREEN
                        | atlas::TREE_ORANGE
                        | atlas::TREE_TEAL
                        | atlas::PINE
                        | atlas::PINE_ORANGE
                        | atlas::PINE_TEAL
                        | atlas::DEAD_TREE
                        | atlas::BUSH
                        | atlas::BERRY_BUSH
                        | atlas::BUSH_FLOWER
                        | atlas::BUSH_FRUIT
                        | atlas::HEDGE
                        | atlas::STUMP
                        | atlas::STUMP_OLD
                        | atlas::STALL_A
                        | atlas::STALL_B
                        | atlas::SIGN
                ) {
                    blob_shadow(fb, px + 2, py + 12, 12, 3);
                }
                fb.sprite(atlas, id, px, py, dl);
            }
        }
    }

    let (fw, fh) = (fb.width(), fb.height());
    let to_screen = move |wx: i32, wy: i32| -> Option<(i32, i32)> {
        let (sx, sy) = (wx * t - cam_x, wy * t - cam_y);
        (sx > -t && sy > -t && sx < fw && sy < fh).then_some((sx, sy))
    };
    let ent_light = dl.max(0.55);

    for critter in &zone.critters {
        if let Some((px, py)) = to_screen(critter.pos.0, critter.pos.1) {
            let id = match critter.kind {
                CritterKind::Chicken => atlas::CHICKEN,
                CritterKind::Sheep => atlas::SHEEP,
                CritterKind::Frog => atlas::FROG,
                CritterKind::Moth => atlas::MOTH,
                CritterKind::Cat => atlas::CAT,
            };
            // Little lives fidget: a one-pixel hop on each critter's own beat.
            let phase = hash2(critter.pos.0, critter.pos.1, 0xC1717) as u64;
            let hop = (app.tick / 4 + phase).is_multiple_of(6) as i32;
            blob_shadow(fb, px + 4, py + 13, 8, 2);
            fb.sprite(atlas, id, px, py - hop, ent_light);
        }
    }

    let active = app.active_quest().map(|q| q.id);
    let night = app.is_night();
    for npc in &zone.npcs {
        let Some((px, py)) = to_screen(npc.pos.0, npc.pos.1) else {
            continue;
        };
        // Anyone standing on a pier with open water at their feet is
        // fishing: they face the river, line cast, and don't glance about.
        let fishing = !night
            && zone.tile(npc.pos.0, npc.pos.1) == Tile::Pier
            && zone.tile(npc.pos.0, npc.pos.1 + 1) == Tile::Water;
        // Folk turn to face the player when spoken-to distance, and breathe
        // with a slow one-pixel sway on their own rhythm.
        let (dx, dy) = (app.player.0 - npc.pos.0, app.player.1 - npc.pos.1);
        let dir = if dx.abs() + dy.abs() == 1 {
            facing_cell((dx, dy)) // turn to face a visitor within talking reach
        } else if night || fishing {
            0 // asleep — or watching the bobber — facing down
        } else {
            // A slow idle gaze: they glance about on their own unhurried beat.
            (hash2(npc.pos.0, npc.pos.1, 0x9A2E).wrapping_add((app.tick / 55) as u32) % 4) as u16
        };
        let phase = npc.name.len() as u64;
        let sway = (app.tick / 12 + phase).is_multiple_of(5) as i32;
        blob_shadow(fb, px + 3, py + 13, 10, 2);
        // At night the folk are abed: draw them a shade dimmer, and never with
        // a quest marker — nobody's handing out errands in their sleep.
        let light = if night { ent_light * 0.82 } else { ent_light };
        fb.sprite(atlas, npc_sprite(npc) + dir, px, py + sway, light);
        if fishing {
            // The cast line, rod tip to bobber, out on the water below.
            fb.sprite(atlas, atlas::ROD_CAST, px, py + TILE as i32, light);
        }
        if night {
            // A little z drifting up from a sleeping head.
            let bob = ((app.tick / 14 + phase) % 3) as i32;
            font::text(fb, px + 9, py - 5 - bob, "z", (188, 200, 224), 1);
        } else if npc.quest.is_some() && npc.quest == active {
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

    {
        // The player rides their glide position, not the tile grid.
        let (px, py) = (ppx - cam_x, ppy - cam_y);
        let dir = facing_cell(app.facing);
        // A fresh step plays the stride — but only the default look has a baked
        // walk-cycle; the others walk on their idle facing, turning to look
        // where they go. Standing still always rests on the idle.
        let walking = app.tick.saturating_sub(app.walked_at) < 4;
        let id = if walking && app.player_char == 0 {
            atlas::PLAYER_WALK + dir * 2 + ((app.tick / 2) % 2) as u16
        } else {
            app.player_look().cast + dir
        };
        blob_shadow(fb, px + 3, py + 13, 10, 2);
        fb.sprite(atlas, id, px, py, ent_light);
    }

    ambient_life(fb, atlas, app, cam_x, cam_y, dl);
    if let Some(kind) = zone.weather {
        weather(fb, kind, app.tick, dl);
    }
}

/// A place-name banner that slides down when you arrive somewhere new, holds a
/// moment, then fades away — "~ Whispering Woods ~".
fn zone_banner(fb: &mut Frame, app: &App) {
    use crate::app::BANNER_TICKS;
    let Some((name, until)) = &app.banner else {
        return;
    };
    let remaining = until.saturating_sub(app.tick);
    let age = BANNER_TICKS.saturating_sub(remaining);
    // Descend into place over the first breaths, fade out over the last.
    let y = if age < 10 {
        20 - ((10 - age) as i32 * 3)
    } else {
        20
    };
    let alpha = if remaining < 12 {
        (remaining as i32 * 255 / 12).clamp(0, 255) as u32
    } else {
        255
    };
    let text = format!("~ {name} ~");
    let tw = font::text_width(&text, 1);
    let cx = fb.width() / 2;
    let (bx, bw) = (cx - tw / 2 - 10, tw + 20);
    fb.fill_a(bx, y - 3, bw, 15, (24, 20, 14), (alpha * 82 / 100) as u8);
    fb.fill_a(bx, y - 3, bw, 1, PANEL_BORDER, alpha as u8);
    fb.fill_a(bx, y + 11, bw, 1, PANEL_BORDER, alpha as u8);
    let tc = (
        (240 * alpha / 255) as u8,
        (210 * alpha / 255) as u8,
        (150 * alpha / 255) as u8,
    );
    font::text_center(fb, cx, y, &text, tc, 1);
}

/// Tile → (opaque base sprite, transparent overlay). The one place tile
/// appearance lives.
fn tile_sprites(
    tile: Tile,
    x: i32,
    y: i32,
    tick: u64,
    zone: &Zone,
    lantern_lit: bool,
) -> (u16, Option<u16>) {
    let h = hash2(x, y, zone.seed);
    let (zone_id, interior) = (zone.id, zone.interior);
    // Indoors freestanding things sit on bare earth (cave stone, cellar
    // dirt); outdoors they sit on whatever the biome grows.
    let ground = if interior {
        interior_earth(zone_id, h)
    } else {
        ground_base(zone, h)
    };
    match tile {
        Tile::Grass => {
            let mut decor = grass_decor(h, zone_id);
            // Decor tufts sway along with their taller tall-grass cousins.
            if let Some(partner) = decor.and_then(sway_partner)
                && (((tick / 12) as i32) + x) % 2 == 0
            {
                decor = Some(partner);
            }
            (ground_base(zone, h), decor)
        }
        Tile::TallGrass => {
            // Each biome's encounter grass sways between its own two frames.
            let (a, b) = match zone_id {
                1 => (atlas::WOODS_TUFT_A, atlas::WOODS_TUFT_B),
                3 => (atlas::TUFT_SNOW_A, atlas::TUFT_SNOW_B),
                _ => (atlas::SPROUT, atlas::SPROUT_ALT),
            };
            let sway = (((tick / 12) as i32) + x) % 2 == 0;
            (ground_base(zone, h), Some(if sway { a } else { b }))
        }
        Tile::Flower => {
            // Flowers are transparent overlays, so they bloom on any ground.
            // In the village, a flower ringed by flowers grows into a dense
            // bed (the bed tiles carry village grass, so only there).
            let petals = |t: Tile| t == Tile::Flower;
            let ring = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter(|&&(dx, dy)| petals(zone.tile(x + dx, y + dy)))
                .count();
            if zone_id == 0 && ring >= 3 {
                let beds = [
                    atlas::FLOWERBED_RED,
                    atlas::FLOWERBED_WHITE,
                    atlas::FLOWERBED_BLUE,
                ];
                (beds[(h % 3) as usize], None)
            } else {
                let singles = [
                    atlas::FLOWER_O_OVER,
                    atlas::FLOWER_W_OVER,
                    atlas::FLOWER_B_OVER,
                ];
                (ground_base(zone, h), Some(singles[(h % 3) as usize]))
            }
        }
        Tile::Tree => {
            // Each zone grows its own woods: broadleaf orchards around the
            // village, deep mixed pines (and the odd dead snag) in the
            // Whispering Woods, hardy conifers up by the Hearthspire.
            const VILLAGE: [u16; 6] = [
                atlas::TREE_GREEN,
                atlas::TREE_GREEN,
                atlas::TREE_ORANGE,
                atlas::TREE_TEAL,
                atlas::TREE_GREEN,
                atlas::PINE,
            ];
            // The woods run dark: pines and teal crowns, no cheery orchard
            // greens past the first bend.
            const WOODS: [u16; 6] = [
                atlas::PINE,
                atlas::PINE_TEAL,
                atlas::PINE_TEAL,
                atlas::TREE_TEAL,
                atlas::PINE,
                atlas::TREE_GREEN,
            ];
            const RIVER: [u16; 6] = [
                atlas::TREE_GREEN,
                atlas::TREE_TEAL,
                atlas::PINE,
                atlas::TREE_GREEN,
                atlas::TREE_ORANGE,
                atlas::PINE_TEAL,
            ];
            const CRAGS: [u16; 6] = [
                atlas::PINE,
                atlas::PINE_TEAL,
                atlas::PINE,
                atlas::TREE_TEAL,
                atlas::PINE_ORANGE,
                atlas::PINE,
            ];
            let mix = match zone_id {
                0 => VILLAGE,
                1 => WOODS,
                2 => RIVER,
                _ => CRAGS,
            };
            // Dead snags stand among the living — a rare sight up on the
            // Hearthspire, but ever more common the deeper into the
            // Whispering Woods you walk.
            let snag_in = match zone_id {
                1 => 3 + (x / 60).clamp(0, 3) as u32,
                3 => 1,
                _ => 0,
            };
            let id = if h % 19 < snag_in {
                atlas::DEAD_TREE
            } else {
                mix[(h % 6) as usize]
            };
            (ground_base(zone, h), Some(id))
        }
        Tile::Bush => {
            let id = match h % 9 {
                0 | 1 => atlas::BERRY_BUSH,
                2 => atlas::BUSH_FLOWER,
                3 => atlas::BUSH_FRUIT,
                _ => atlas::BUSH,
            };
            (ground_base(zone, h), Some(id))
        }
        Tile::Water => {
            // Water laps around pilings and hulls: a slow ripple beside
            // anything built out over it (piers, moored boats).
            let built = |t: Tile| matches!(t, Tile::Pier | Tile::Facade(_));
            let ripples = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .any(|&(dx, dy)| built(zone.tile(x + dx, y + dy)));
            if ripples {
                let frame = ((tick / 10 + h as u64) % 4) as u16;
                return (water_frame(x, y, tick), Some(atlas::RIPPLE + frame));
            }
            // A lily pad drifts here and there, where water meets the banks.
            let pad = h.is_multiple_of(29)
                && [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .any(|&(dx, dy)| matches!(zone.tile(x + dx, y + dy), Tile::Reed | Tile::Sand));
            (water_frame(x, y, tick), pad.then_some(atlas::LILY_PAD))
        }
        Tile::WaterRock => {
            let id = match h % 5 {
                0 => atlas::ROCK_GREY_C,
                1 => atlas::ROCK_MOSSY_GREY,
                _ => atlas::ROCK_GREY_B,
            };
            (water_frame(x, y, tick), Some(id))
        }
        Tile::Reed => {
            // Riverbank reeds stand on the bank beside the water.
            let sway = (((tick / 14) as i32) + x) % 2 == 0;
            let id = match h % 5 {
                0 => atlas::LILY,
                1 => atlas::SHRUB_SMALL,
                _ if sway => atlas::SPROUT_ALT,
                _ => atlas::SPROUT,
            };
            (ground_base(zone, h), Some(id))
        }
        Tile::Bridge => (atlas::BRIDGE, None),
        Tile::Pier => {
            // Boardwalk planking; the cell whose south side meets open water
            // shows its piling tops, so every pier stands on visible legs.
            let end = !matches!(zone.tile(x, y + 1), Tile::Pier);
            (
                water_frame(x, y, tick),
                Some(if end {
                    atlas::PIER_END
                } else {
                    atlas::PIER_PLANK
                }),
            )
        }
        // Under the ground, "path" is the floor itself: cave stone strewn
        // with what grows and glitters in the dark, cellar earth kept
        // tidier (it is somebody's pantry, after all).
        Tile::Path if zone_id == ECHO_CAVE => {
            let decor = match h % 31 {
                0 => Some(atlas::CRYSTAL_VIOLET),
                1 => Some(atlas::CRYSTAL_AMBER),
                2 => Some(atlas::SHROOMS_PALE),
                3 => Some(atlas::SHROOMS_TALL),
                4 | 5 => Some(atlas::CAVE_ROCKS),
                6 => Some(atlas::STAL_SMALL),
                7 => Some(atlas::OLD_BONES),
                _ => None,
            };
            (interior_earth(zone_id, h), decor)
        }
        Tile::Path if zone_id == STOREHOUSE_CELLAR => {
            let decor = match h % 43 {
                0 => Some(atlas::COBWEB_B),
                1 => Some(atlas::SHROOMS_PALE),
                2 => Some(atlas::PEBBLE),
                _ => None,
            };
            (interior_earth(zone_id, h), decor)
        }
        Tile::Path => (
            if h.is_multiple_of(9) {
                atlas::PATH_ALT
            } else {
                atlas::PATH
            },
            None,
        ),
        Tile::Plaza => (
            if h.is_multiple_of(7) {
                atlas::COBBLE_ALT
            } else {
                atlas::COBBLE
            },
            None,
        ),
        Tile::Sand => (atlas::SAND, None),
        Tile::Wall => {
            let decor = if !interior {
                // Outside walls grow a sprig of ivy here and there.
                h.is_multiple_of(7).then_some(atlas::IVY)
            } else {
                // Inside, each room dresses its own walls: cobwebs gather in
                // the cellar and the old storehouse, the lived-in houses hang
                // little framed pictures on the back wall — the one you see
                // face-on, with floor at its feet. The Library hangs its own.
                let back_wall = matches!(zone.tile(x, y + 1), Tile::Floor | Tile::Rug);
                match zone_id {
                    STOREHOUSE_CELLAR => h.is_multiple_of(5).then_some(atlas::COBWEB_A),
                    STOREHOUSE => (back_wall && h.is_multiple_of(4)).then_some(atlas::COBWEB_A),
                    // Nobody has swept the abandoned houses in a long while.
                    WOODS_RUIN | WOODS_LODGE => h.is_multiple_of(3).then_some(atlas::COBWEB_A),
                    BAKERY | SORREL_COTTAGE | CARPENTER_HOUSE | TILLY_COTTAGE if back_wall => {
                        match h % 11 {
                            0 => Some(atlas::FRAME_TEAL),
                            1 => Some(atlas::FRAME_AMBER),
                            2 => Some(atlas::FRAME_SMALL),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            };
            (wall_base(zone_id, h), decor)
        }
        // A building prefab cell: the atlas cell rides in the tile itself
        // (see the Tile::Facade docs), drawn over whatever grows beneath so
        // the roofline's transparent corners show grass, not void. What
        // "beneath" is comes from the nearest ground outside the prefab, so
        // the plaza fountain stands on cobbles and a lane-side house on its
        // lane, never on a stray patch of grass.
        Tile::Facade(cell) | Tile::FacadeDoor(cell) => {
            (facade_ground(zone, x, y, h, tick), Some(cell))
        }
        // Alternate exterior wall builds: sprigs of ivy here and there, same
        // as plain Wall — just a different building material underneath.
        Tile::WallStone => (atlas::WALL_STONE, h.is_multiple_of(7).then_some(atlas::IVY)),
        Tile::WallPlaster => (
            atlas::WALL_PLASTER,
            h.is_multiple_of(7).then_some(atlas::IVY),
        ),
        Tile::Roof => (atlas::ROOF, None),
        Tile::RoofSlate => (atlas::ROOF_SLATE, None),
        Tile::RoofCream => (atlas::ROOF_CREAM, None),
        Tile::Door => (wall_base(zone_id, h), Some(atlas::DOOR)),
        // A facade's shut door: always the same weathered arch, regardless of
        // the wall it's set in — nobody's meant to look closely, just walk by.
        Tile::DoorClosed => (atlas::WALL, Some(atlas::DOOR_ARCH)),
        Tile::Floor => (interior_floor(zone_id, h), None),
        Tile::Fence => {
            // Fences follow their run: rails along a row, posts-and-rails up
            // a column, and a stout post at corners, junctions and ends.
            let fency = |t: Tile| matches!(t, Tile::Fence | Tile::Gate);
            let row = fency(zone.tile(x - 1, y)) || fency(zone.tile(x + 1, y));
            let col = fency(zone.tile(x, y - 1)) || fency(zone.tile(x, y + 1));
            let id = match (row, col) {
                (true, false) => atlas::FENCE,
                (false, true) => atlas::FENCE_V,
                _ => atlas::FENCE_POST,
            };
            (ground_base(zone, h), Some(id))
        }
        Tile::Cliff => {
            // Crag bands: weathered stone, cracks, and the odd fallen boulder.
            let overlay = match h % 10 {
                0 | 1 => Some(atlas::ROCK_GREY),
                2 | 3 => Some(atlas::ROCK_GREY_B),
                4 => Some(atlas::ROCK_GREY_C),
                5 => Some(atlas::ROCK_BROWN),
                6 | 7 => Some(atlas::CRACK_A),
                8 => Some(atlas::CRACK_B),
                _ => None,
            };
            (atlas::STONE, overlay)
        }
        Tile::Rock if zone_id == ECHO_CAVE => {
            // The cave's rocky bounds: stalagmites among the fallen stone.
            let id = match h % 8 {
                0 | 1 => atlas::STAL_TALL_A,
                2 => atlas::STAL_TALL_B,
                3 => atlas::CAVE_ROCKS_MOSS,
                4 => atlas::STAL_SMALL,
                _ => atlas::CAVE_ROCKS,
            };
            (ground, Some(id))
        }
        Tile::Rock => {
            let id = match (zone_id, h % 8) {
                // Mossgrown in the damp woods, riverstone by the ford,
                // snow-capped up on the Hearthspire approach.
                (1, 0..=2) => atlas::ROCK_MOSSY_GREY,
                (1, 3) => atlas::ROCK_MOSSY_BROWN,
                (2, 0..=1) => atlas::NA_STONE,
                (3, 0..=3) => atlas::SNOWROCK_A,
                (3, 4..=5) => atlas::SNOWROCK_B,
                (_, 0) => atlas::ROCK_GREY_B,
                (_, 1) => atlas::ROCK_GREY_C,
                (_, 2) => atlas::ROCK_MOSSY_GREY,
                _ => atlas::ROCK_GREY,
            };
            (ground, Some(id))
        }
        Tile::Stall => (
            atlas::COBBLE,
            Some(if h.is_multiple_of(2) {
                atlas::STALL_A
            } else {
                atlas::STALL_B
            }),
        ),
        Tile::Awning => (
            atlas::COBBLE,
            Some(if h.is_multiple_of(2) {
                atlas::AWNING_GREEN
            } else {
                atlas::AWNING_ORANGE
            }),
        ),
        Tile::Campfire => {
            let flame = if (tick / 6).is_multiple_of(2) {
                atlas::CAMPFIRE_A
            } else {
                atlas::CAMPFIRE_B
            };
            let base = if interior {
                ground
            } else {
                paved_base(zone, x, y, atlas::PATH_ALT)
            };
            (base, Some(flame))
        }
        Tile::Lantern => {
            let id = if !lantern_lit {
                atlas::TORCH_UNLIT
            } else if (tick / 8).is_multiple_of(2) {
                atlas::TORCH_LIT_A
            } else {
                atlas::TORCH_LIT_B
            };
            let base = if interior {
                interior_floor(zone_id, h)
            } else {
                paved_base(zone, x, y, atlas::PATH)
            };
            (base, Some(id))
        }
        Tile::Gate => (atlas::PATH, Some(atlas::GATE)),
        Tile::Sign => (ground, Some(atlas::SIGN)),
        Tile::Void => (atlas::VOID, None),
        // Rugs are drawn edge-aware by `rug_detail`, so any patch of them
        // reads as one carpet instead of a grid of doormats.
        Tile::Rug => (atlas::FLOOR, None),
        Tile::Bookshelf => (interior_floor(zone_id, h), Some(atlas::BOOKSHELF)),
        Tile::Shelf => {
            // The bakery's shelves are its shop counter, stocked with wares;
            // everywhere else a shelf is just a shelf.
            let id = if zone_id == BAKERY {
                [
                    atlas::COUNTER_PLATES,
                    atlas::COUNTER_JARS,
                    atlas::COUNTER_JUGS,
                    atlas::COUNTER_BOTTLES,
                ][(h % 4) as usize]
            } else {
                atlas::SHELF
            };
            (interior_floor(zone_id, h), Some(id))
        }
        Tile::Table => {
            // Round café tables in the bakery and cottages, a cluttered
            // workbench in Alder's workshop, plain boards elsewhere.
            let id = match zone_id {
                BAKERY | SORREL_COTTAGE | TILLY_COTTAGE => atlas::TABLE_ROUND,
                CARPENTER_HOUSE => [
                    atlas::BENCH_JUGS,
                    atlas::BENCH_PLATES,
                    atlas::BENCH_WOOD,
                    atlas::BENCH_JARS,
                ][(h % 4) as usize],
                STOREHOUSE => atlas::BENCH_WOOD,
                _ => atlas::TABLE,
            };
            (interior_floor(zone_id, h), Some(id))
        }
        Tile::Stool => (interior_floor(zone_id, h), Some(atlas::STOOL)),
        Tile::BedHead => {
            let id = match zone_id {
                SORREL_COTTAGE => atlas::BED_STRIPE_HEAD,
                TILLY_COTTAGE => atlas::BED_CREAM_HEAD,
                _ => atlas::BED_HEAD,
            };
            (interior_floor(zone_id, h), Some(id))
        }
        Tile::BedFoot => {
            let id = match zone_id {
                SORREL_COTTAGE => atlas::BED_STRIPE_FOOT,
                TILLY_COTTAGE => atlas::BED_CREAM_FOOT,
                _ => atlas::BED_FOOT,
            };
            (interior_floor(zone_id, h), Some(id))
        }
        Tile::Hearth => {
            // The bakery cooks on a proper range; every other hearth is an
            // open fire, cosily flickering.
            if zone_id == BAKERY {
                let id = if h.is_multiple_of(2) {
                    atlas::STOVE_A
                } else {
                    atlas::STOVE_B
                };
                (interior_floor(zone_id, h), Some(id))
            } else {
                let flame = if (tick / 6).is_multiple_of(2) {
                    atlas::HEARTH_A
                } else {
                    atlas::HEARTH_B
                };
                (interior_floor(zone_id, h), Some(flame))
            }
        }
        Tile::Barrel => {
            // Down in the cellar the "barrels" are Granny-era preserve urns.
            let id = if zone_id == STOREHOUSE_CELLAR {
                atlas::URN
            } else {
                atlas::BARREL
            };
            (furniture_base(zone, x, y), Some(id))
        }
        Tile::Crate => {
            // One of the workshop's crates is no crate at all — it's the anvil.
            let id = if zone_id == CARPENTER_HOUSE && h.is_multiple_of(4) {
                atlas::ANVIL
            } else {
                atlas::CRATE
            };
            (furniture_base(zone, x, y), Some(id))
        }
        Tile::Herb => (ground, Some(atlas::HERB)),
        Tile::Chest => (furniture_base(zone, x, y), Some(atlas::CHEST)),
        Tile::Runestone => (furniture_base(zone, x, y), Some(atlas::RUNESTONE)),
        Tile::Window => {
            // Interior windows keep the hand-pixeled glow (it feeds the
            // sunbeam light-shaft effect); exterior windows get some variety.
            let sprite = if interior {
                atlas::WINDOW
            } else {
                match h % 3 {
                    0 => atlas::WINDOW_ROUND,
                    1 => atlas::WINDOW_SQUARE,
                    _ => atlas::WINDOW,
                }
            };
            (wall_base(zone_id, h), Some(sprite))
        }
        Tile::Painting => {
            // The gallery hangs a mix: framed canvases, a map, a mirror.
            let id = match h % 7 {
                0 | 1 => atlas::PAINTING_MEADOW,
                2 => atlas::PAINTING_MAP,
                3 => atlas::MIRROR,
                _ => atlas::PAINTING,
            };
            (wall_base(zone_id, h), Some(id))
        }
        Tile::Plant => {
            let id = match h % 3 {
                0 => atlas::POT_PLANT_A,
                1 => atlas::POT_PLANT_B,
                _ => atlas::PLANT,
            };
            (interior_floor(zone_id, h), Some(id))
        }
        Tile::Pedestal => (interior_floor(zone_id, h), Some(atlas::PEDESTAL)),
        Tile::Piano => (interior_floor(zone_id, h), Some(atlas::PIANO)),
        Tile::Clock => (interior_floor(zone_id, h), Some(atlas::CLOCK)),
    }
}

/// Fixtures that stand in paved places (the lantern, the campfire) sit on
/// cobbles when the square around them is cobbled, packed earth otherwise.
fn paved_base(zone: &Zone, x: i32, y: i32, default: u16) -> u16 {
    let paved = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .any(|&(dx, dy)| zone.tile(x + dx, y + dy) == Tile::Plaza);
    if paved { atlas::COBBLE } else { default }
}

/// What shows through a prefab cell's transparent pixels: the ground the
/// building actually stands on. A prefab stamp replaces the tiles beneath it,
/// so the original ground is gone — instead, walk outward in growing rings
/// until something that isn't part of a prefab turns up, and match it. The
/// plaza fountain gets cobbles, a roadside house its road, a moored boat the
/// river it floats on, everything else the zone's grass.
fn facade_ground(zone: &Zone, x: i32, y: i32, h: u32, tick: u64) -> u16 {
    for r in 1..=4i32 {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dy.abs()) != r {
                    continue; // only the ring's edge — inner cells were seen
                }
                match zone.tile(x + dx, y + dy) {
                    Tile::Facade(_) | Tile::FacadeDoor(_) => continue,
                    Tile::Plaza => {
                        return if h.is_multiple_of(7) {
                            atlas::COBBLE_ALT
                        } else {
                            atlas::COBBLE
                        };
                    }
                    Tile::Path => {
                        return if h.is_multiple_of(9) {
                            atlas::PATH_ALT
                        } else {
                            atlas::PATH
                        };
                    }
                    Tile::Sand => return atlas::SAND,
                    Tile::Water | Tile::WaterRock | Tile::Pier => {
                        return water_frame(x, y, tick);
                    }
                    _ => return ground_base(zone, h),
                }
            }
        }
    }
    ground_base(zone, h)
}

/// What freestanding clutter sits on: the biome's ground outdoors, and
/// indoors whatever the room is floored with — bare stone or earth in the
/// cave and cellar, that room's boards everywhere else — so a barrel never
/// brings its own square of wrong floor.
fn furniture_base(zone: &Zone, x: i32, y: i32) -> u16 {
    let h = hash2(x, y, zone.seed);
    if !zone.interior {
        return ground_base(zone, h);
    }
    let earthen = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .any(|&(dx, dy)| zone.tile(x + dx, y + dy) == Tile::Path);
    if earthen {
        interior_earth(zone.id, h)
    } else {
        interior_floor(zone.id, h)
    }
}

/// Each room's floor: pale scrubbed planks in the bakery, rough boards in
/// the workshop and storehouse, warm sandstone under the Library's halls,
/// homely mid-brown boards everywhere else.
fn interior_floor(zone_id: usize, h: u32) -> u16 {
    match zone_id {
        BAKERY => atlas::FLOOR_LIGHT,
        CARPENTER_HOUSE | STOREHOUSE | WOODS_RUIN | WOODS_LODGE => atlas::FLOOR_BOARDS,
        GREAT_LIBRARY => {
            if h.is_multiple_of(11) {
                atlas::SANDSTONE_B
            } else {
                atlas::SANDSTONE_A
            }
        }
        _ => atlas::FLOOR,
    }
}

/// Bare ground underfoot indoors: speckled cave stone in the Echo Cave,
/// packed cellar earth below the storehouse, plain trodden dirt elsewhere.
fn interior_earth(zone_id: usize, h: u32) -> u16 {
    let family = match zone_id {
        ECHO_CAVE => [
            atlas::CAVE_FLOOR_A,
            atlas::CAVE_FLOOR_B,
            atlas::CAVE_FLOOR_C,
        ],
        STOREHOUSE_CELLAR => [
            atlas::EARTH_FLOOR_A,
            atlas::EARTH_FLOOR_B,
            atlas::EARTH_FLOOR_C,
        ],
        _ => return atlas::PATH,
    };
    match h % 7 {
        0 => family[1],
        1 => family[2],
        _ => family[0],
    }
}

/// What a wall is built of, room by room: quarried stone blocks down in the
/// cellar (cracked and veined here and there), timber everywhere else.
fn wall_base(zone_id: usize, h: u32) -> u16 {
    if zone_id == STOREHOUSE_CELLAR {
        return match h % 9 {
            0 => atlas::STONE_WALL_CRACK,
            1 => atlas::STONE_WALL_VEIN,
            _ => atlas::STONE_WALL,
        };
    }
    atlas::WALL
}

/// The zone's ground — what "grass" is there: spring green in the village,
/// dark forest floor in the woods, wet marsh meadow by the river, snowfield
/// on the Hearthspire approach. Mostly plain, with sprinkled richer variants.
fn ground_base(zone: &Zone, h: u32) -> u16 {
    let family = match zone.id {
        1 => [
            atlas::WOODS_FLOOR,
            atlas::WOODS_FLOOR_B,
            atlas::WOODS_FLOOR_C,
        ],
        2 => [atlas::MARSH, atlas::MARSH_B, atlas::MARSH_C],
        3 => [atlas::SNOW, atlas::SNOW_B, atlas::SNOW_C],
        _ => {
            return match h % 19 {
                0 => atlas::GRASS_ALT,
                // The mottled patch only reads as sunlit growth in
                // daylight; in dim places it would look like a stain.
                1 if zone.daylight > 0.5 => atlas::GRASS_MOTTLED,
                _ => atlas::GRASS,
            };
        }
    };
    match h % 13 {
        0 => family[1],
        1 => family[2],
        _ => family[0],
    }
}

/// The other frame of a two-frame swaying tuft, if this decor sways.
fn sway_partner(id: u16) -> Option<u16> {
    Some(match id {
        atlas::SPROUT => atlas::SPROUT_ALT,
        atlas::SPROUT_ALT => atlas::SPROUT,
        atlas::TUFT_DEEP_A => atlas::TUFT_DEEP_B,
        atlas::TUFT_DEEP_B => atlas::TUFT_DEEP_A,
        atlas::TUFT_MARSH_A => atlas::TUFT_MARSH_B,
        atlas::TUFT_MARSH_B => atlas::TUFT_MARSH_A,
        atlas::TUFT_SNOW_A => atlas::TUFT_SNOW_B,
        atlas::TUFT_SNOW_B => atlas::TUFT_SNOW_A,
        _ => return None,
    })
}

/// Sprinkle non-blocking décor over plain ground — each biome grows its own
/// (garden flowers in the village, ferns and logs in the deep woods, bog
/// clumps in the marsh, ice and drift-tufts — and the odd old bone — in
/// the snow).
fn grass_decor(h: u32, zone_id: usize) -> Option<u16> {
    if zone_id == 3 && h % 179 == 3 {
        return Some(atlas::BONE);
    }
    // Village gardens: the odd tall sunflower or rose bush between the homes.
    if zone_id == 0 && h % 61 == 5 {
        return Some(if h.is_multiple_of(2) {
            atlas::SUNFLOWER_TALL
        } else {
            atlas::ROSEBUSH
        });
    }
    if h % 5 != 3 {
        return None;
    }
    // The swaying tuft pairs are reserved for TallGrass, so encounter grass
    // stays tellable from mere decoration.
    const MIXES: [[u16; 6]; 4] = [
        [
            atlas::SPROUT_ALT,
            atlas::FLOWER_SMALL_A,
            atlas::SUNFLOWER,
            atlas::FLOWER_SMALL_B,
            atlas::SPROUT,
            atlas::SHRUB_SMALL,
        ],
        [
            atlas::MUSHROOM,
            atlas::FERN,
            atlas::STUMP_OLD,
            atlas::MUSHROOM_TALL,
            atlas::LOG_NA,
            atlas::SHRUB_SMALL,
        ],
        [
            atlas::TUFT_MARSH_A,
            atlas::BOGBERRY,
            atlas::PEBBLE,
            atlas::SHRUB_SMALL,
            atlas::TUFT_MARSH_B,
            atlas::FLOWER_SMALL_B,
        ],
        [
            atlas::PEBBLE,
            atlas::SNOWROCK_B,
            atlas::ICE_A,
            atlas::ICE_B,
            atlas::ICE_A,
            atlas::SNOWROCK_A,
        ],
    ];
    Some(MIXES[zone_id.min(3)][(h / 16) as usize % 6])
}

/// A curl of smoke over a campfire: three specks on staggered clocks, each
/// rising, drifting a little, and thinning to nothing before it loops.
fn smoke_wisp(fb: &mut Frame, px: i32, py: i32, tick: u64, seed: u32, dl: f32) {
    let grey = shade((198, 196, 202), dl.max(0.45));
    for k in 0..3i32 {
        let t = ((tick + hash2(k, 7, seed) as u64) % 26) as i32;
        let x = px + 7 + ((hash2(k, t / 5, seed) % 3) as i32 - 1);
        let y = py + 3 - t / 2;
        let fade = (140 - t * 5).max(0) as u8;
        fb.blend(x, y, grey, fade);
        if t < 13 {
            fb.blend(x + 1, y, grey, fade / 2);
        }
    }
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
    matches!(t, Tile::Water | Tile::WaterRock | Tile::Bridge | Tile::Pier)
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

/// A one-pixel darker rim where paving meets greenery, so paths and plazas
/// read as worn into the grass instead of painted on top.
#[allow(clippy::too_many_arguments)]
fn path_rim(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
    rim: (u8, u8, u8),
) {
    let t = TILE as i32;
    let grassy = |tile: Tile| {
        matches!(
            tile,
            Tile::Grass | Tile::TallGrass | Tile::Flower | Tile::Tree | Tile::Bush
        )
    };
    let rim = shade(rim, dl);
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

/// A woven carpet over the floorboards: warm weave inside, a pale hem only
/// along the edges that meet bare floor, so rug patches read as one rug.
fn rug_detail(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
) {
    let t = TILE as i32;
    // The Library lays a deep moss-green carpet; homes keep the warm red.
    let (warp, weft) = if zone.id == GREAT_LIBRARY {
        ((74, 112, 84), (60, 94, 70))
    } else {
        ((156, 72, 60), (134, 58, 48))
    };
    let weave = shade(warp, dl);
    fb.fill(px, py, t, t, weave);
    for k in 0..6 {
        let h = hash2(wx * 7 + k, wy * 3 + k, 0x2A6);
        fb.set(
            px + (h % 16) as i32,
            py + ((h >> 8) % 16) as i32,
            shade(weft, dl),
        );
    }
    let hem = shade((222, 198, 152), dl);
    let bare = |tile: Tile| tile != Tile::Rug;
    if bare(zone.tile(wx, wy - 1)) {
        fb.fill(px, py, t, 2, hem);
    }
    if bare(zone.tile(wx, wy + 1)) {
        fb.fill(px, py + t - 2, t, 2, hem);
    }
    if bare(zone.tile(wx - 1, wy)) {
        fb.fill(px, py, 2, t, hem);
    }
    if bare(zone.tile(wx + 1, wy)) {
        fb.fill(px + t - 2, py, 2, t, hem);
    }
}

/// Light falling through a window: a floor tile within a few steps below a
/// window catches a soft shaft of it — warm and strong by day, a pale cool
/// sliver by night — fading with distance from the pane.
#[allow(clippy::too_many_arguments)]
fn sunbeam(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
    tick: u64,
) {
    let mut dist = 0;
    for d in 1..=3 {
        match zone.tile(wx, wy - d) {
            Tile::Window => {
                dist = d;
                break;
            }
            // Only open floor lets the light travel down to here.
            Tile::Floor | Tile::Rug => {}
            _ => return,
        }
    }
    if dist == 0 {
        return;
    }
    let t = dl.clamp(0.0, 1.0);
    // Warm gold in daylight, a cool blue sliver after dark.
    let col = (
        (150.0 + 105.0 * t) as u8,
        (170.0 + 62.0 * t) as u8,
        (210.0 - 40.0 * t) as u8,
    );
    let strength = (16.0 + 58.0 * t) as i32 / dist;
    let tw = TILE as i32;
    // A gentle shimmer so the shaft feels alive.
    let shimmer = (((tick / 10 + wx as u64) % 5) as i32 - 2).max(0);
    fb.fill_a(px + 3, py, tw - 6, tw, col, (strength).clamp(4, 90) as u8);
    fb.fill_a(
        px + 5 + shimmer,
        py,
        tw - 12,
        tw,
        col,
        (strength / 2).clamp(3, 60) as u8,
    );
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

/// Any of the roof tiles, regardless of which building's palette it wears —
/// so a slate roof's eave doesn't cut a false edge against its own ridge.
fn is_roof(tile: Tile) -> bool {
    matches!(tile, Tile::Roof | Tile::RoofSlate | Tile::RoofCream)
}

/// Shingle lines across roof tiles, and a darker eave along the roof's edge,
/// so houses read as buildings instead of flat color slabs. `shingle`/`eave`
/// are matched to whichever roof palette (amber/slate/cream) called this.
#[allow(clippy::too_many_arguments)]
fn roof_detail(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
    shingle: (u8, u8, u8),
    eave: (u8, u8, u8),
) {
    let t = TILE as i32;
    for row in [3, 7, 11, 15] {
        fb.fill_a(px, py + row, t, 1, shade(shingle, dl), 110);
    }
    let eave = shade(eave, dl);
    if !is_roof(zone.tile(wx, wy - 1)) {
        fb.fill(px, py, t, 1, eave);
    }
    if !is_roof(zone.tile(wx, wy + 1)) {
        fb.fill(px, py + t - 1, t, 1, eave);
    }
    if !is_roof(zone.tile(wx - 1, wy)) {
        fb.fill(px, py, 1, t, eave);
    }
    if !is_roof(zone.tile(wx + 1, wy)) {
        fb.fill(px + t - 1, py, 1, t, eave);
    }
}

fn lighten(c: (u8, u8, u8)) -> (u8, u8, u8) {
    (
        c.0.saturating_add(46),
        c.1.saturating_add(46),
        c.2.saturating_add(46),
    )
}

/// A roofline's exposed top edge rises above its own tile in a triangular
/// ridge — tallest at the middle of the contiguous run, tapering to nothing
/// at its ends — so the building reads as having real height instead of
/// being a flat painted rectangle. Purely a paint trick: it overdraws into
/// the tile row above (already drawn, since rows paint top-to-bottom), and
/// doesn't touch collision, so nothing about walkability changes.
#[allow(clippy::too_many_arguments)]
fn roof_peak(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
    color: (u8, u8, u8),
) {
    if is_roof(zone.tile(wx, wy - 1)) {
        return; // not the roof's top edge — another roof row already covers it
    }
    let exposed = |x: i32| is_roof(zone.tile(x, wy)) && !is_roof(zone.tile(x, wy - 1));
    let mut left = wx;
    while exposed(left - 1) {
        left -= 1;
    }
    let mut right = wx;
    while exposed(right + 1) {
        right += 1;
    }
    let span = (right - left + 1) as f32;
    let half = (span / 2.0).max(1.0);
    let dist = (wx as f32 - (left + right) as f32 / 2.0).abs();
    let rise = (8.0 * (1.0 - dist / half).clamp(0.0, 1.0)).round() as i32;
    if rise <= 0 {
        return;
    }
    let t = TILE as i32;
    let hi = shade(lighten(color), dl);
    let base = shade(color, dl);
    for dy in 0..rise {
        let c = if dy == 0 { hi } else { base };
        fb.fill(px, py - rise + dy, t, 1, c);
    }
}

/// A soft shadow on the ground where it meets a building wall, so the
/// structure looks like it's actually sitting on the earth rather than
/// pasted over it.
fn building_shadow(
    fb: &mut Frame,
    zone: &crate::world::map::Zone,
    wx: i32,
    wy: i32,
    px: i32,
    py: i32,
    dl: f32,
) {
    let walled = |t: Tile| {
        matches!(
            t,
            Tile::Wall | Tile::WallStone | Tile::WallPlaster | Tile::Door | Tile::DoorClosed
        )
    };
    if walled(zone.tile(wx, wy - 1)) {
        fb.fill_a(px, py, TILE as i32, 3, shade((20, 24, 16), dl), 90);
    }
}

/// An undiscovered runestone catches the light now and then — a small cyan
/// twinkle beside the stone. Once its rune is in the journal, it rests.
fn stone_glimmer(fb: &mut Frame, app: &App, wx: i32, wy: i32, px: i32, py: i32) {
    let found = crate::world::zones::runestone_id(app.zone_idx, (wx, wy))
        .map(|id| app.has_flag(&sides::runestone_flag(id)))
        .unwrap_or(true);
    if found {
        return;
    }
    let phase = (app.tick / 5).wrapping_add(hash2(wx, wy, 0x57A2) as u64) % 16;
    if phase >= 4 {
        return;
    }
    let (gx, gy) = (px + 12, py + 2);
    let c = (170, 230, 240);
    fb.set(gx, gy, c);
    if phase < 2 {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            fb.set(gx + dx, gy + dy, c);
        }
    }
}

/// Butterflies bob around fixed spots in the world by day, and now and then a
/// bird crosses the sky. Small lives, big difference. Takes the camera's
/// pixel origin, so the little lives scroll as smoothly as the ground.
fn ambient_life(fb: &mut Frame, atlas: &Atlas, app: &App, cam_x: i32, cam_y: i32, dl: f32) {
    use crate::world::map::{MAP_H, MAP_W};
    if dl <= 0.45 || app.zone().interior {
        return; // butterflies and birds are day folk, and outdoor folk at that
    }
    let t = app.tick as f32;
    for i in 0..120i32 {
        let ax = (hash2(i, 11, 0xB77F) % (MAP_W as u32 * TILE as u32)) as i32;
        let ay = (hash2(i, 12, 0xB77F) % (MAP_H as u32 * TILE as u32)) as i32;
        let x = ax + ((t / 9.0 + i as f32).sin() * 14.0) as i32 - cam_x;
        let y = ay + ((t / 6.0 + i as f32 * 2.0).cos() * 8.0) as i32 - cam_y;
        if x < -(TILE as i32) || y < -(TILE as i32) || x >= fb.width() || y >= fb.height() {
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
            fb.width() - p
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

/// Pixel offset of the player's drawn position from their logical tile: eases
/// from the departure square to zero across one step's length. Warps and
/// gate-crossings (any hop longer than a step) arrive instantly.
fn player_glide(app: &App) -> (i32, i32) {
    let (dx, dy) = (
        app.prev_player.0 - app.player.0,
        app.prev_player.1 - app.player.1,
    );
    if dx.abs() > 1 || dy.abs() > 1 {
        return (0, 0);
    }
    // A diagonal step covers √2 ground, so the shell spaces its held repeats a
    // little wider; stretch the glide to match, or the sprite reaches the tile
    // early and freezes until the next step — a visible lurch every diagonal.
    let mut step_ticks = crate::app::STEP_SECS / crate::app::TICK_SECS;
    if dx != 0 && dy != 0 {
        step_ticks *= crate::app::DIAGONAL_STRETCH;
    }
    // The step departed part-way through a tick (`walk_subtick`), so measure
    // the glide from that exact moment — not the rounded tick — to start it
    // squarely at the departure square instead of a pixel or two ahead.
    let now = app.tick as f32 + app.subtick;
    let started = app.walked_at as f32 + app.walk_subtick;
    let left = 1.0 - ((now - started) / step_ticks).clamp(0.0, 1.0);
    (
        (dx as f32 * left * TILE as f32).round() as i32,
        (dy as f32 * left * TILE as f32).round() as i32,
    )
}

fn water_frame(x: i32, y: i32, tick: u64) -> u16 {
    if ((x + y * 3) as i64 + (tick / 12) as i64).rem_euclid(2) == 0 {
        atlas::WATER_A
    } else {
        atlas::WATER_B
    }
}

/// Atlas cell offset for a facing step vector (down, up, left, right —
/// the order the cast strips were baked in).
fn facing_cell(step: (i32, i32)) -> u16 {
    match step {
        (0, -1) => 1,
        (-1, 0) => 2,
        (1, 0) => 3,
        _ => 0, // down, and the shrug for anything unexpected
    }
}

/// An NPC's idle cell, facing down (add `facing_cell` for the other ways):
/// every named quest-giver and side character has a fixed sprite, and any
/// future stranger falls back to a townsfolk look. Nothing here ever lands
/// on a char-select sprite (Boy, Child, ManGreen, Woman — members 0, 4, 6,
/// 16), so no NPC mirrors a traveller the player can be.
///
/// Quest ids no longer line up with atlas member order (the beginner-quest
/// expansion inserted new quests between the originals without renumbering
/// the atlas), so every NPC is matched by name rather than derived from
/// `npc.quest`.
fn npc_sprite(npc: &Npc) -> u16 {
    // Any future nameless stranger borrows one of these, none of them a face
    // from the char-select screen.
    const TOWNSFOLK: [u16; 3] = [
        atlas::CAST_VILLAGER_M,
        atlas::CAST_VILLAGER2,
        atlas::CAST_VILLAGER3,
    ];
    match npc.name {
        "Elder Rowan" => atlas::CAST_MASTER,
        "Baker Poppy" => atlas::CAST_OLDMAN2,
        "Well-keeper Bram" => atlas::CAST_VILLAGER_M,
        "Forager Maren" => atlas::CAST_VILLAGE6,
        "Ferryman Wick" => atlas::CAST_VILLAGER2,
        "Fisher Juniper" => atlas::CAST_EGGBOY,
        "Hermit Morrow" => atlas::CAST_MONK,
        "Archivist Elm" => atlas::CAST_INSPECTOR,
        "The Stone Golem" => atlas::CAST_STATUE,
        "Sage Alderly" => atlas::CAST_OLDMAN3,
        "Granny Sorrel" => atlas::CAST_OLDWOMAN,
        "Old Nettle" => atlas::CAST_GREENMAN,
        "Carpenter Alder" => atlas::CAST_HUNTER,
        "Under-librarian Twill" => atlas::CAST_NOBLE,
        // Child, ManGreen and Woman are the player's to wear now, so the two
        // NPCs who used to own those sprites take spare villager looks.
        "Shepherd Ambrose" => atlas::CAST_VILLAGER3, // was ManGreen
        "Wren" => atlas::CAST_VILLAGER4,             // was Child
        "Hen-keeper Tilly" => atlas::CAST_PRINCESS,  // was Woman
        // The beginner-quest expansion's cast, its own tail block in the atlas.
        "Tansy" => atlas::CAST_TANSY,
        "Watchman Fitch" => atlas::CAST_FITCH,
        "Toll-keeper Hobb" => atlas::CAST_HOBB,
        "Cartographer Reed" => atlas::CAST_REED,
        "Pip" => atlas::CAST_PIP,
        "Basket-weaver Briar" => atlas::CAST_BRIAR,
        "Hollow-keeper Yew" => atlas::CAST_YEW,
        "Woodward Sable" => atlas::CAST_SABLE,
        "Dockhand Fenn" => atlas::CAST_FENN,
        "Net-mender Sil" => atlas::CAST_SIL,
        "Scribe Faye" => atlas::CAST_FAYE,
        _ => TOWNSFOLK[npc.name.bytes().map(usize::from).sum::<usize>() % 3],
    }
}

// ── HUD bars ───────────────────────────────────────────────────────────────

fn top_bar(fb: &mut Frame, app: &App, dl: f32) {
    fb.fill_a(0, 0, fb.width(), 17, (22, 19, 14), 215);
    daylight_icon(fb, 9, 8, dl);
    let mut x = font::text_lg(fb, 20, 3, app.zone().name, (226, 208, 168)) + 8;
    for q in QUESTS.iter().filter(|q| q.zone == app.zone().id) {
        diamond(fb, x + 3, 8, 3, GOLD, app.completed.contains(&q.id));
        x += 10;
    }
    // The counters keep the small face so the roomier zone name never
    // squeezes them out; they're glanced at, not read.
    let runes = format!("runes {}/{}", app.completed.len(), QUESTS.len());
    let w = font::text_width(&runes, 1);
    font::text(fb, fb.width() - w - 6, 5, &runes, (178, 162, 140), 1);
    // The time of day, left of the rune count — only outdoors, where the
    // shared sky actually holds sway (interiors keep their own steady hour).
    // On the narrowest windows a long zone name may leave it no room; the
    // sun/moon icon still tells the hour, so it simply sits this one out.
    if !app.zone().interior {
        let phase = app.phase().label();
        let pw = font::text_width(phase, 1);
        let px = fb.width() - w - pw - 20;
        if px > x + 8 {
            font::text(fb, px, 5, phase, (196, 186, 208), 1);
        }
    }
}

fn bottom_bar(fb: &mut Frame, app: &App) {
    let cols = ((fb.width() - 12) / font::GLYPH_LG - 1).max(20) as usize;
    let (lines, color) = match &app.toast {
        Some((msg, _)) => (font::wrap(msg, cols), (255, 224, 150)),
        None => {
            let near = app.zone().npcs.iter().find(|n| {
                (n.pos.0 - app.player.0).abs() <= 1 && (n.pos.1 - app.player.1).abs() <= 1
            });
            let hint = match near {
                Some(npc) => format!(
                    "e talk to {} . c cast . q journal . f hint . esc rest",
                    npc.name
                ),
                None => {
                    "arrows move . e talk . c cast . q journal . g grimoire . f hint . esc".into()
                }
            };
            (font::wrap(&hint, cols), DIM)
        }
    };
    let lines = &lines[..lines.len().min(3)];
    let bar_h = 5 + lines.len() as i32 * font::LINE_LG;
    let bottom = fb.height();
    fb.fill_a(0, bottom - bar_h, fb.width(), bar_h, (22, 19, 14), 215);
    for (i, line) in lines.iter().enumerate() {
        font::text_lg(
            fb,
            6,
            bottom - bar_h + 3 + i as i32 * font::LINE_LG,
            line,
            color,
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
    let (w, h) = (fb.w as i64 / 8, fb.h as i64 / 8); // 8px particle grid
    match kind {
        Weather::Petals => {
            // Petal count follows the width so wider screens don't thin out.
            let petals = (w * 26 / 60).max(20);
            for i in 0..petals {
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
            for gx in 0..(fb.w as i64 / 4) {
                let col = hash2(gx as i32, 0, 0x0A1D);
                for (layer, speed) in [(1u32, 5i64), (2, 8)] {
                    if col.wrapping_mul(layer) % 10 < 3 {
                        let offset = (col.wrapping_mul(layer.wrapping_add(7)) % 500) as i64;
                        let span = fb.h as i64 + 40;
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
    let (w, h) = (fb.w as i64 / 8, fb.h as i64 / 8);
    let width_factor = w as f32 / 60.0; // keep density steady as the view widens
    let count = (30.0 * width_factor * (0.25 + 0.75 * dim)).max(6.0) as i64;
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
    fb.fill_a(0, 0, fb.width(), fb.height(), (8, 6, 4), 90);
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
        // The roomy reading face, unless the title is a long one (a book's
        // full name, a wordy fizzle) — those keep the small face and fit.
        let tw = font::text_width_lg(title);
        if tw + 24 <= w {
            fb.fill(x + 8, y - 4, tw + 8, 17, PANEL_BG);
            font::text_lg(fb, x + 12, y - 1, title, WARM);
        } else {
            let tw = font::text_width(title, 1);
            fb.fill(x + 8, y - 3, tw + 8, 13, PANEL_BG);
            font::text(fb, x + 12, y, title, WARM, 1);
        }
    }
    (x + 6, y + 10, w - 12, h - 16)
}

fn centered_panel(fb: &mut Frame, w: i32, h: i32, title: &str) -> (i32, i32, i32, i32) {
    let (px, py) = ((fb.width() - w) / 2, (fb.height() - h) / 2);
    panel(fb, px, py, w, h, title)
}

fn draw_lines(fb: &mut Frame, x: i32, y: i32, lines: &[(String, (u8, u8, u8))]) -> i32 {
    let mut cy = y;
    for (line, color) in lines {
        font::text(fb, x, cy, line, *color, 1);
        cy += 9;
    }
    cy
}

/// Same, in the one-and-a-half reading face (menus and dialogue).
fn draw_lines_lg(fb: &mut Frame, x: i32, y: i32, lines: &[(String, (u8, u8, u8))]) -> i32 {
    let mut cy = y;
    for (line, color) in lines {
        font::text_lg(fb, x, cy, line, *color);
        cy += font::LINE_LG;
    }
    cy
}

// ── dialogue ───────────────────────────────────────────────────────────────

/// How much prose fits one dialogue page at the larger reading face —
/// `Dialogue::new` re-flows its pages against these, so nothing authored
/// ever falls off the bottom of the box.
pub const DIALOGUE_COLS: usize = 30;
pub const DIALOGUE_ROWS: usize = 8;

fn dialogue(fb: &mut Frame, atlas: &Atlas, app: &App, d: &Dialogue) {
    let (w, h) = (460, 126);
    let x = (fb.width() - w) / 2;
    let y = fb.height() - h - 8;
    let (ix, iy, iw, ih) = panel(fb, x, y, w, h, &d.speaker);

    // Portrait: the speaker's sprite, nice and big.
    let portrait = if matches!(d.kind, DialogueKind::Book) {
        Some(atlas::BOOKSHELF)
    } else if matches!(d.kind, DialogueKind::Stone) {
        Some(atlas::RUNESTONE)
    } else if d.speaker == "Signpost" {
        Some(atlas::SIGN)
    } else {
        app.zone()
            .npcs
            .iter()
            .find(|n| n.name == d.speaker)
            .map(npc_sprite)
    };
    let text_x = if let Some(id) = portrait {
        fb.sprite_scaled(atlas, id, ix + 2, iy + (ih - 64) / 2, 4, 1.0);
        ix + 74
    } else {
        ix + 4
    };

    let page = &d.pages[d.page.min(d.pages.len() - 1)];
    let shown: String = page.chars().take(d.revealed).collect();
    // Pages arrive pre-flowed to DIALOGUE_COLS (see Dialogue::new), so the
    // wrap here re-derives the same lines while the reveal is mid-word.
    let lines: Vec<_> = font::wrap(&shown, DIALOGUE_COLS)
        .into_iter()
        .map(|l| (l, BODY))
        .collect();
    draw_lines_lg(fb, text_x, iy + 2, &lines[..lines.len().min(DIALOGUE_ROWS)]);

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
    let (ix, iy, iw, ih) = centered_panel(fb, 464, 252, "Journal");
    let cols = (iw / font::GLYPH_LG - 1) as usize;
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

    let satchel = items::satchel(&app.completed);
    let carried = sides::carried(&app.flags);
    if !satchel.is_empty() || !carried.is_empty() {
        lines.push((String::new(), DIM));
        let mut owned: Vec<&str> = satchel.iter().map(|i| i.name()).collect();
        owned.extend(carried);
        let fish_line;
        if app.fish > 0 {
            fish_line = format!("{} fish met", app.fish);
            owned.push(&fish_line);
        }
        push(&format!("Satchel: {}", owned.join(" . ")), WARM, &mut lines);
    }

    // Side business underway: little memory-aids, never demands.
    let notes = sides::journal_notes(&app.flags);
    if !notes.is_empty() {
        lines.push((String::new(), DIM));
        for note in notes {
            push(&format!("~ {note}"), (168, 186, 200), &mut lines);
        }
    }

    let max = 17usize;
    draw_lines_lg(fb, ix + 4, iy + 2, &lines[..lines.len().min(max)]);

    // The tallies live on a fixed footer line (small face, like the
    // grimoire's), so however long the hints run they never push it out.
    let stones_found = stones::found(&app.flags);
    let stones_part = if stones_found > 0 {
        format!(
            " . runestones {}/{}",
            stones_found,
            stones::RUNESTONES.len()
        )
    } else {
        String::new() // the stones stay a secret until the first is found
    };
    let footer = format!(
        "Runes mastered: {}/12 . grimoire {}/{} (g){} . esc close",
        app.completed.len(),
        app.grimoire.len(),
        wilds::WILDS.len(),
        stones_part,
    );
    let w = font::text_width(&footer, 1);
    font::text(fb, ix + iw - w - 2, iy + ih - 8, &footer, DIM, 1);
}

// ── wild runes: encounters & the grimoire ──────────────────────────────────

fn encounter(fb: &mut Frame, rune_id: u8, selected: usize, phase: EncounterPhase) {
    let rune = wilds::wild(rune_id);
    let (ix, iy, iw, ih) = centered_panel(fb, 420, 180, "Something stirs in the grass");
    let cols = (iw / GLYPH - 1) as usize;

    let mut lines: Vec<(String, (u8, u8, u8))> = Vec::new();
    for l in font::wrap(rune.stir, cols) {
        lines.push((l, BODY));
    }
    lines.push((String::new(), DIM));
    lines.push((format!("~ {} ~", rune.name), GOLD));
    lines.push((String::new(), DIM));

    match phase {
        EncounterPhase::Asking => {
            for l in font::wrap(rune.prompt, cols) {
                lines.push((l, WARM));
            }
            lines.push((String::new(), DIM));
            for (i, option) in rune.options.iter().enumerate() {
                let (marker, color) = if i == selected {
                    ("> ", GOLD)
                } else {
                    ("  ", DIM)
                };
                for (j, l) in font::wrap(option, cols - 2).into_iter().enumerate() {
                    let lead = if j == 0 { marker } else { "  " };
                    lines.push((format!("{lead}{l}"), color));
                }
            }
        }
        EncounterPhase::Caught => {
            lines.push((
                "The rune settles happily into your grimoire!".into(),
                (160, 210, 140),
            ));
            lines.push((String::new(), DIM));
            for l in font::wrap(rune.lore, cols) {
                lines.push((l, BODY));
            }
        }
        EncounterPhase::Fizzled => {
            lines.push((
                "fzzt — not quite! The rune giggles and skitters off.".into(),
                (230, 170, 120),
            ));
            lines.push((String::new(), DIM));
            lines.push(("No harm done. It'll rustle around here again.".into(), DIM));
        }
    }
    draw_lines(fb, ix + 4, iy + 2, &lines[..lines.len().min(16)]);

    let footer = match phase {
        EncounterPhase::Asking => "up/down choose . enter answer . esc slip away",
        _ => "enter . back to the grass",
    };
    let w = font::text_width(footer, 1);
    font::text(fb, ix + iw - w - 2, iy + ih - 8, footer, DIM, 1);
}

fn grimoire(fb: &mut Frame, app: &App) {
    let (ix, iy, iw, ih) = centered_panel(fb, 464, 252, "Grimoire - wild runes of the road");
    // Names only, two columns per zone — the lore is read at catch time —
    // so all four zones fit on one page.
    let mut y = iy + 2;
    for zone in 0..=3 {
        font::text_lg(fb, ix + 4, y, app.zones[zone].name, WARM);
        y += font::LINE_LG + 1;
        let runes = wilds::in_zone(zone);
        for (i, rune) in runes.iter().enumerate() {
            let x = ix + 10 + (i as i32 % 2) * (iw / 2);
            if app.grimoire.contains(&rune.id) {
                font::text(fb, x, y, &format!("* {}", rune.name), BODY, 1);
            } else {
                font::text(fb, x, y, ". ???", (110, 102, 88), 1);
            }
            if i % 2 == 1 {
                y += 11;
            }
        }
        if runes.len() % 2 == 1 {
            y += 11;
        }
        y += 8;
    }
    let footer = format!(
        "{}/{} inscribed . wild runes live in tall grass . esc",
        app.grimoire.len(),
        wilds::WILDS.len()
    );
    let w = font::text_width(&footer, 1);
    font::text(fb, ix + iw - w - 2, iy + ih - 8, &footer, DIM, 1);
}

// ── casting & results ──────────────────────────────────────────────────────

fn casting(fb: &mut Frame, app: &App) {
    let (ix, iy, iw, _) = centered_panel(fb, 280, 72, "Casting");
    let spin = ['|', '/', '-', '\\'][(app.tick / 2) as usize % 4];
    let phrase = quests::WEAVING[(app.tick / 24) as usize % quests::WEAVING.len()];
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

fn paused(fb: &mut Frame, app: &App, selected: usize) {
    let (ix, iy, _iw, _) = centered_panel(fb, 320, 112, "A moment's rest");
    let speed = ["Slow", "Normal", "Fast"][app.text_speed.min(2)];
    let speed_label = format!("Text speed: < {speed} >");
    let labels = ["Back to the road", &speed_label, "Save & sleep (quit)"];
    for (i, label) in labels.iter().enumerate() {
        let on = i == selected;
        let c = if on { GOLD } else { DIM };
        let marker = if on { "> " } else { "  " };
        font::text_lg(
            fb,
            ix + 10,
            iy + 8 + i as i32 * 20,
            &format!("{marker}{label}"),
            c,
        );
    }
}

/// Resting by a campfire: the world falls away into ember-dark, a curl of
/// sparks rises, and a scrap of Rust lore keeps you company until you wake.
fn resting(fb: &mut Frame, app: &App, lore_idx: usize, t: u32, wake: DayPhase) {
    fb.clear((10, 8, 9));

    let (cx, bottom) = (fb.width() / 2, fb.height());
    // A low bank of embers along the bottom, breathing with the tick.
    for i in 0..70i32 {
        let base = cx - 60 + (hash2(i, 1, 0xF16E) % 120) as i32;
        let flick = (app.tick / 4 + hash2(i, 2, 0xF16E) as u64) % 5;
        let c = [(210, 96, 40), (240, 150, 60), (150, 54, 30)][(flick % 3) as usize];
        fb.blend(base, bottom - 14 + (flick as i32 % 3), c, 150);
    }
    // Sparks drifting up from the fire.
    for k in 0..16i32 {
        let phase = (app.tick + hash2(k, 3, 0xF16E) as u64) % 60;
        let x =
            cx - 40 + (hash2(k, 4, 0xF16E) % 80) as i32 + ((phase as f32 / 8.0).sin() * 6.0) as i32;
        let y = bottom - 20 - phase as i32 * 3;
        let fade = (200 - phase as i32 * 3).max(0) as u8;
        fb.blend(x, y, (250, 190, 110), fade);
    }

    let lore = &lore::LORE[lore_idx.min(lore::LORE.len() - 1)];
    let (ix, iy, iw, ih) = centered_panel(fb, 400, 150, "~ resting by the fire ~");
    let cols = (iw / GLYPH - 1) as usize;
    font::text(fb, ix + 4, iy + 4, lore.voice, GOLD, 1);
    let lines: Vec<_> = font::wrap(lore.text, cols)
        .into_iter()
        .map(|l| (l, (222, 210, 188)))
        .collect();
    draw_lines(fb, ix + 4, iy + 20, &lines);

    let footer = match wake {
        DayPhase::Night => "enter — drift off (you'll wake to night)",
        _ => "enter — sleep till the morning",
    };
    let w = font::text_width(footer, 1);
    font::text(fb, ix + iw - w - 2, iy + ih - 8, footer, DIM, 1);

    // Fade in from black over the first breaths, so it reads as drifting off.
    let scrim = 235u32.saturating_sub(t * 22).min(235) as u8;
    fb.fill_a(0, 0, fb.width(), fb.height(), (0, 0, 0), scrim);
}

/// Choosing your traveller: a row of portraits to leaf through, and a name to
/// spell out. Reached from the title's "A new journey".
fn char_select(fb: &mut Frame, atlas: &Atlas, app: &App, idx: usize, name: &str) {
    fb.clear((13, 15, 22));
    fireflies(fb, app.tick, 0.0);
    let cx = fb.width() / 2;

    font::text_center(fb, cx, 26, "WHO WILL YOU BE?", (240, 205, 120), 2);

    // The roster, portraits centered as a row; the chosen one lifts and glows.
    let roster = &atlas::PLAYABLE;
    let n = roster.len() as i32;
    let step = 78;
    let row_x = cx - (n - 1) * step / 2;
    let py = 78;
    for (i, who) in roster.iter().enumerate() {
        let on = i == idx;
        let x = row_x + i as i32 * step;
        let scale = if on { 5 } else { 4 };
        let sprite_w = TILE as i32 * scale;
        let lift = if on { ((app.tick / 10) % 2) as i32 } else { 0 };
        let sx = x - sprite_w / 2;
        let sy = py + (if on { 0 } else { 8 }) - lift;
        if on {
            // A soft plinth of light under the chosen one.
            fb.fill_a(
                sx - 4,
                sy - 4,
                sprite_w + 8,
                sprite_w + 8,
                (60, 54, 40),
                120,
            );
            fb.fill(sx - 4, sy - 4, sprite_w + 8, 2, GOLD);
            fb.fill(sx - 4, sy + sprite_w + 2, sprite_w + 8, 2, GOLD);
        }
        let light = if on { 1.0 } else { 0.55 };
        fb.sprite_scaled(atlas, who.cast, sx, sy, scale, light);
    }

    // The chosen look's label.
    font::text_center(fb, cx, py + 100, roster[idx].look, WARM, 1);

    // The name field, with a soft blinking cursor. Empty until the player
    // types — no borrowed placeholder name, just an inviting blank and cursor.
    let label = "Name: ";
    let full = format!("{label}{name}");
    let fw = font::text_width(&full, 1);
    let nx = cx - fw / 2;
    let ny = py + 122;
    font::text(fb, nx, ny, label, (150, 150, 170), 1);
    let vx = font::text(fb, nx + font::text_width(label, 1), ny, name, GOLD, 1);
    if (app.tick / 8).is_multiple_of(2) {
        fb.fill(vx + 1, ny, 5, 8, WARM);
    }

    // A gentle nudge (e.g. trying to set off unnamed) borrows the toast line.
    if let Some((msg, _)) = &app.toast {
        font::text_center(fb, cx, fb.height() - 36, msg, (255, 224, 150), 1);
    }

    font::text_center(
        fb,
        cx,
        fb.height() - 22,
        "arrows pick a look, type a name, enter to begin",
        (110, 105, 95),
        1,
    );
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

    let cx = fb.width() / 2;
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
        fb.height() - 20,
        "up/down choose . enter set off . keep a code editor handy",
        (110, 105, 95),
        1,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::zones::zones;

    // A traveller you pick at char-select must never meet a copy of themselves
    // wandering the world: no NPC may wear a sprite from the playable roster.
    // The step-glide must start squarely at the departure square and, for a
    // held diagonal, ease over the stretched interval instead of arriving
    // early and freezing until the next step.
    #[test]
    fn glide_starts_full_and_stretches_diagonals() {
        let mut app = App::new();
        // A step from (5,5) east to (6,5), captured the instant it happened.
        app.tick = 10;
        app.walked_at = 10;
        app.walk_subtick = 0.0;
        app.subtick = 0.0;
        app.prev_player = (5, 5);
        app.player = (6, 5);
        // At the step moment the drawn position is the whole departure offset.
        assert_eq!(player_glide(&app), (-(TILE as i32), 0));

        // 2.4 ticks on (one straight step's worth): a straight move has landed.
        app.tick = 12;
        app.subtick = 0.4;
        assert_eq!(player_glide(&app), (0, 0));

        // A diagonal step of the same age is still in flight — its glide is
        // stretched by DIAGONAL_STRETCH, so it hasn't frozen on the tile yet.
        app.prev_player = (5, 5);
        app.player = (6, 6);
        let (gx, gy) = player_glide(&app);
        assert!(gx < 0 && gx == gy, "diagonal glide still moving: {gx},{gy}");
    }

    #[test]
    fn no_npc_shares_a_playable_sprite() {
        let playable: Vec<u16> = atlas::PLAYABLE.iter().map(|p| p.cast).collect();
        for zone in zones() {
            for npc in &zone.npcs {
                let sprite = npc_sprite(npc);
                assert!(
                    !playable.contains(&sprite),
                    "{} in {} wears a playable-only sprite ({sprite})",
                    npc.name,
                    zone.name,
                );
            }
        }
    }
}
