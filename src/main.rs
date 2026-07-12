//! Rune & Road: the game state (`rgame::app`) rendered as 16×16 sprites on a
//! 480×270 framebuffer and integer-scaled up by this thin Macroquad shell.
//!
//! ```sh
//! cargo run
//! ```

use macroquad::audio::{self, PlaySoundParams, Sound};
use macroquad::prelude as mq;

use rgame::app::{App, Key, Screen, SoundEvent, Terrain};
use rgame::checker::Outcome;
use rgame::gfx::{self, Atlas, FB_H, FB_W, Frame};

const TICK_SECS: f32 = rgame::app::TICK_SECS;
/// Held movement keys repeat after this long, every `REPEAT_EVERY`. The delay
/// is kept short so walking picks up quickly once a key is held, while still
/// leaving a hair of margin over a deliberate single tap so one press steps one
/// tile. The pace comes from the game (`app::STEP_SECS`) so the renderer's
/// step-glide covers exactly one repeat: feet and pixels agree.
const REPEAT_AFTER: f32 = 0.14;
const REPEAT_EVERY: f32 = rgame::app::STEP_SECS;
/// Walking both axes at once covers √2 ground per step, so diagonal repeats
/// come a touch slower to keep the traveller honest. Shared with the renderer
/// so the glide stretches by the same factor and never freezes mid-diagonal.
const DIAGONAL_STRETCH: f32 = rgame::app::DIAGONAL_STRETCH;

fn conf() -> mq::Conf {
    mq::Conf {
        window_title: "Rune & Road".into(),
        window_width: (FB_W * 2) as i32,
        window_height: (FB_H * 2) as i32,
        ..Default::default()
    }
}

/// Physical, non-character keys. Letters are read from the OS character stream
/// instead (see the main loop), so the game follows the user's keyboard layout
/// — a Dvorak `h`/`j`/`k`/`l` walks, wherever those keys physically sit.
const KEYMAP: &[(mq::KeyCode, Key)] = &[
    (mq::KeyCode::Up, Key::Up),
    (mq::KeyCode::Down, Key::Down),
    (mq::KeyCode::Left, Key::Left),
    (mq::KeyCode::Right, Key::Right),
    (mq::KeyCode::Enter, Key::Enter),
    (mq::KeyCode::KpEnter, Key::Enter),
    (mq::KeyCode::Escape, Key::Esc),
    (mq::KeyCode::Space, Key::Char(' ')),
    (mq::KeyCode::Backspace, Key::Backspace),
    (mq::KeyCode::PageUp, Key::PageUp),
    (mq::KeyCode::PageDown, Key::PageDown),
];

/// Arrow keys walk the player and so deserve smooth hold-to-repeat. (The vim
/// keys walk too, but repeat via the OS character stream.)
const MOVEMENT: &[mq::KeyCode] = &[
    mq::KeyCode::Up,
    mq::KeyCode::Down,
    mq::KeyCode::Left,
    mq::KeyCode::Right,
];

/// Which way a game key walks, if it's a movement key at all.
fn dir_of(key: Key) -> Option<(i32, i32)> {
    match key {
        Key::Up | Key::Char('k') => Some((0, -1)),
        Key::Down | Key::Char('j') => Some((0, 1)),
        Key::Left | Key::Char('h') => Some((-1, 0)),
        Key::Right | Key::Char('l') => Some((1, 0)),
        _ => None,
    }
}

const MUSIC_VOLUME: f32 = 0.5;
/// The daytime zone loops sit well under the title theme — a playtest found
/// them loud against the night's calm, so day now hums along at nearly the
/// night ambience's level instead of singing over the world.
const DAY_MUSIC_VOLUME: f32 = 0.32;
/// Night ambience stays soft — it wants to feel like the world settling
/// rather than a new song starting.
const NIGHT_VOLUME: f32 = 0.4;
/// The calm melody over the night beds sits softest of all — it should feel
/// like it drifts in from somewhere over the hills.
const NIGHT_THEME_VOLUME: f32 = 0.35;
/// The daytime weather beds are texture, not song: rain and wind held well
/// under the zone chiptune, at the edge of noticing.
const DAY_BED_VOLUME: f32 = 0.25;
const SFX_VOLUME: f32 = 0.7;
/// The hearth is a room-tone, not a song: present the moment you notice it,
/// easy to stop noticing.
const HEARTH_VOLUME: f32 = 0.4;
/// The owl is meant to be *far off* — a soft note under the ambience, never a
/// jump-scare. Kept low on purpose.
const OWL_VOLUME: f32 = 0.28;

/// One looping chiptune per overworld region (`assets/audio/music/`), indexed
/// by `zones::region_of(App::zone_idx)` — interiors stay quiet. See
/// `assets/CREDITS.md` for licensing.
static ZONE_MUSIC: &[&[u8]] = &[
    include_bytes!("../assets/audio/music/emberwick.ogg"),
    include_bytes!("../assets/audio/music/whispering-woods.ogg"),
    include_bytes!("../assets/audio/music/silverford.ogg"),
    include_bytes!("../assets/audio/music/hearthspire.ogg"),
    include_bytes!("../assets/audio/music/mistholm.ogg"),
];

/// A calmer nature ambience per overworld region for after dark, same indexing
/// as `ZONE_MUSIC` — crickets over Emberwick, a living swamp in the Woods,
/// rain on Silverford, wind off the Hearthspire road, waves lapping the
/// Mistholm piers. Swapped in for the daytime loop whenever `App::is_night()`.
/// See `assets/CREDITS.md`.
static NIGHT_MUSIC: &[&[u8]] = &[
    include_bytes!("../assets/audio/music/night/emberwick.ogg"),
    include_bytes!("../assets/audio/music/night/whispering-woods.ogg"),
    include_bytes!("../assets/audio/music/night/silverford.ogg"),
    include_bytes!("../assets/audio/music/night/hearthspire.ogg"),
    include_bytes!("../assets/audio/music/night/mistholm.ogg"),
];

/// The calm night theme ("Dream", from the Ninja Adventure pack — see
/// `assets/CREDITS.md`): one gentle melody looped over *every* outdoor zone
/// after dark, laid on top of that zone's nature bed, so night has real music
/// rather than crickets alone.
static NIGHT_THEME: &[u8] = include_bytes!("../assets/audio/music/night/theme.ogg");

/// By day, the zones whose *drawn* weather makes a sound lay it softly under
/// their chiptune (Ninja Adventure ambience loops, CC0): wind through the
/// Whispering Woods' canopy, the rain that always falls on Silverford, wind
/// off the misty Hearthspire road, far-off surf around the Mistholm isles.
/// Emberwick keeps its clear morning — petals don't sound. Indexed like
/// `ZONE_MUSIC`, `None` where the air is still.
static DAY_BEDS: [Option<&[u8]>; 5] = [
    None,
    Some(include_bytes!(
        "../assets/ninja_adventure/pack/Audio/Sounds/Ambient/Wind2.ogg"
    )),
    Some(include_bytes!(
        "../assets/ninja_adventure/pack/Audio/Sounds/Ambient/Rain.ogg"
    )),
    Some(include_bytes!(
        "../assets/ninja_adventure/pack/Audio/Sounds/Ambient/Wind.ogg"
    )),
    Some(include_bytes!(
        "../assets/ninja_adventure/pack/Audio/Sounds/Ambient/WaveFar.ogg"
    )),
];

/// A lone owl, hooted at random intervals under the night ambience (never by
/// day, never indoors). See `assets/CREDITS.md` for licensing.
static SFX_OWL: &[u8] = include_bytes!("../assets/audio/sfx/owl.ogg");

/// The title/char-select theme — looped while the player is still in the
/// menus, silent everywhere else. See `assets/CREDITS.md` for licensing.
static TITLE_MUSIC: &[u8] = include_bytes!("../assets/audio/music/title.ogg");

/// A soft fireplace crackle looped inside the lived-in rooms
/// (`zones::has_hearth`), so a house isn't dead silent when the zone music
/// stops at the door. See `assets/CREDITS.md` for licensing.
static HEARTH_LOOP: &[u8] = include_bytes!("../assets/audio/music/fireplace.ogg");

static SFX_CAST: &[u8] = include_bytes!("../assets/audio/sfx/cast.ogg");
static SFX_PASS: &[u8] = include_bytes!("../assets/audio/sfx/pass.ogg");
static SFX_FIZZLE: &[u8] = include_bytes!("../assets/audio/sfx/fizzle.ogg");

// ── foley & jingles (see assets/CREDITS.md) ────────────────────────────────
// Footsteps by terrain — two takes each, alternated so a walk never
// metronomes. Kenney *RPG Audio*, CC0.
static STEPS_SOFT: [&[u8]; 2] = [
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep00.ogg"),
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep01.ogg"),
];
static STEPS_EARTH: [&[u8]; 2] = [
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep02.ogg"),
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep03.ogg"),
];
static STEPS_SAND: [&[u8]; 2] = [
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep04.ogg"),
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep05.ogg"),
];
static STEPS_WOOD: [&[u8]; 2] = [
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep06.ogg"),
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep07.ogg"),
];
static STEPS_STONE: [&[u8]; 2] = [
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep08.ogg"),
    include_bytes!("../assets/kenney/audio/rpg-audio/footstep09.ogg"),
];
// The world's small noises (Kenney *RPG Audio*, CC0): door creaks on warp,
// the cellar chest's groan, a coin-ish chime for a keepsake, page turns.
static SFX_DOOR: &[u8] = include_bytes!("../assets/kenney/audio/rpg-audio/doorOpen_2.ogg");
static SFX_CHEST: &[u8] = include_bytes!("../assets/kenney/audio/rpg-audio/creak2.ogg");
static SFX_COINS: &[u8] = include_bytes!("../assets/kenney/audio/rpg-audio/handleCoins.ogg");
static SFX_PAGE: &[u8] = include_bytes!("../assets/kenney/audio/rpg-audio/bookFlip2.ogg");
// A pot on the range for a dish leaving Poppy's ovens.
static SFX_POT: &[u8] = include_bytes!("../assets/kenney/audio/rpg-audio/metalPot2.ogg");
// Menu blips (Kenney *UI Audio*, CC0).
static SFX_BLIP: &[u8] = include_bytes!("../assets/kenney/audio/ui-audio/click1.ogg");
// Jingles at the milestones (Kenney *Music Jingles*, CC0): a steel-drum
// sparkle when a rune joins the grimoire, an 8-bit gleam for a runestone.
static SFX_RUNE: &[u8] =
    include_bytes!("../assets/kenney/audio/music-jingles/Steel jingles/jingles_STEEL07.ogg");
static SFX_STONE: &[u8] =
    include_bytes!("../assets/kenney/audio/music-jingles/8-Bit jingles/jingles_NES00.ogg");
// A soft pizzicato flourish when a grimoire rune is cast from the ring.
static SFX_RUNE_CAST: &[u8] =
    include_bytes!("../assets/kenney/audio/music-jingles/Pizzicato jingles/jingles_PIZZI00.ogg");
// The encounter's sting and the campfire's rest theme — the shelf's unused
// Junkala tracks (CC0), looped softly for as long as their screens hold.
static ENCOUNTER_MUSIC: &[u8] =
    include_bytes!("../assets/audio/shelf/chiptunes-action-level-1.ogg");
static REST_MUSIC: &[u8] = include_bytes!("../assets/audio/shelf/chiptune-adventures-stage-2.ogg");

const STEP_VOLUME: f32 = 0.22;
const FOLEY_VOLUME: f32 = 0.55;
const BLIP_VOLUME: f32 = 0.30;
const JINGLE_VOLUME: f32 = 0.60;
const ENCOUNTER_VOLUME: f32 = 0.30;
const REST_VOLUME: f32 = 0.35;

/// The rest menu's sound option, as a master gain: off, quiet, full.
fn sound_gain(level: usize) -> f32 {
    [0.0, 0.45, 1.0][level.min(2)]
}

/// A one-shot cue, raised on the frame the screen first shows casting/pass/
/// fizzle and silent otherwise — `on_key`/`on_tick` never touch audio, so the
/// shell derives cues by diffing `app.screen` across frames.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cue {
    None,
    Casting,
    Pass,
    Fizzle,
}

fn cue_for(screen: &Screen) -> Cue {
    match screen {
        Screen::Casting { .. } => Cue::Casting,
        Screen::CastResult {
            outcome: Outcome::Pass { .. },
            ..
        } => Cue::Pass,
        Screen::CastResult { .. } => Cue::Fizzle,
        _ => Cue::None,
    }
}

/// Pick a crisp integer pixel-scale and the framebuffer size that fills a
/// window of `(sw, sh)` edge-to-edge with no letterbox. The scale is the
/// largest at which the native 480×270 layout still fits the window in *both*
/// directions, so the minimum-size clamps below never push the picture past
/// the window edges (rounding the scale up used to crop the HUD's top/bottom
/// bars on heights like a fractionally-scaled 1440p); the framebuffer then
/// grows in whichever direction the window is wider, so ultrawide and
/// superultrawide displays simply see more of the world instead of black bars.
fn fit(sw: f32, sh: f32) -> (i32, usize, usize) {
    let scale = (sw / FB_W as f32).min(sh / FB_H as f32).floor().max(1.0) as i32;
    let fb_w = ((sw / scale as f32).ceil() as usize).max(FB_W);
    let fb_h = ((sh / scale as f32).ceil() as usize).max(FB_H);
    (scale, fb_w, fb_h)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every window at least as large as the native 480×270 layout must be
    /// covered edge-to-edge (no letterbox) with the whole picture on-screen:
    /// overhang stays under one pixel-scale step, so the HUD's top/bottom
    /// bars can never land outside the window. Sweeps sizes up to 4K,
    /// including the fractional-DPI heights (e.g. 3440×1440 at 150% → a
    /// 2293×960 logical window) that the old round-up scale used to crop.
    #[test]
    fn fit_never_crops_windows_at_native_size_or_larger() {
        for sw in (FB_W..=3840).step_by(7) {
            for sh in (FB_H..=2160).step_by(7) {
                let (scale, fb_w, fb_h) = fit(sw as f32, sh as f32);
                let (dw, dh) = (fb_w * scale as usize, fb_h * scale as usize);
                assert!(fb_w >= FB_W && fb_h >= FB_H, "{sw}x{sh} shrank the layout");
                assert!(dw >= sw && dh >= sh, "{sw}x{sh} left a letterbox");
                assert!(
                    dw - sw < scale as usize && dh - sh < scale as usize,
                    "{sw}x{sh} at scale {scale} overhangs by {}x{}",
                    dw - sw,
                    dh - sh,
                );
            }
        }
    }
}

#[macroquad::main(conf)]
async fn main() {
    let atlas = Atlas::load();
    let mut fb = Frame::new();
    let mut app = App::new();

    let mut zone_music: Vec<Sound> = Vec::with_capacity(ZONE_MUSIC.len());
    for bytes in ZONE_MUSIC {
        zone_music.push(
            audio::load_sound_from_bytes(bytes)
                .await
                .expect("zone music is baked into the binary"),
        );
    }
    let mut night_music: Vec<Sound> = Vec::with_capacity(NIGHT_MUSIC.len());
    for bytes in NIGHT_MUSIC {
        night_music.push(
            audio::load_sound_from_bytes(bytes)
                .await
                .expect("night ambience is baked into the binary"),
        );
    }
    let night_theme = audio::load_sound_from_bytes(NIGHT_THEME)
        .await
        .expect("night theme is baked into the binary");
    let mut day_beds: Vec<Option<Sound>> = Vec::with_capacity(DAY_BEDS.len());
    for bytes in DAY_BEDS {
        day_beds.push(match bytes {
            Some(bytes) => Some(
                audio::load_sound_from_bytes(bytes)
                    .await
                    .expect("day weather beds are baked into the binary"),
            ),
            None => None,
        });
    }
    let sfx_owl = audio::load_sound_from_bytes(SFX_OWL)
        .await
        .expect("owl sfx is baked into the binary");
    let title_music = audio::load_sound_from_bytes(TITLE_MUSIC)
        .await
        .expect("title music is baked into the binary");
    let hearth_loop = audio::load_sound_from_bytes(HEARTH_LOOP)
        .await
        .expect("hearth loop is baked into the binary");
    let sfx_cast = audio::load_sound_from_bytes(SFX_CAST)
        .await
        .expect("cast sfx is baked into the binary");
    let sfx_pass = audio::load_sound_from_bytes(SFX_PASS)
        .await
        .expect("pass sfx is baked into the binary");
    let sfx_fizzle = audio::load_sound_from_bytes(SFX_FIZZLE)
        .await
        .expect("fizzle sfx is baked into the binary");
    // The foley shelf: footsteps per terrain, the world's small noises, the
    // menu blip and the milestone jingles — all baked into the binary.
    async fn sfx(bytes: &'static [u8]) -> Sound {
        audio::load_sound_from_bytes(bytes)
            .await
            .expect("foley is baked into the binary")
    }
    let mut steps: Vec<(Terrain, [Sound; 2])> = Vec::new();
    for (terrain, takes) in [
        (Terrain::Soft, STEPS_SOFT),
        (Terrain::Earth, STEPS_EARTH),
        (Terrain::Sand, STEPS_SAND),
        (Terrain::Wood, STEPS_WOOD),
        (Terrain::Stone, STEPS_STONE),
    ] {
        steps.push((terrain, [sfx(takes[0]).await, sfx(takes[1]).await]));
    }
    let sfx_door = sfx(SFX_DOOR).await;
    let sfx_chest = sfx(SFX_CHEST).await;
    let sfx_coins = sfx(SFX_COINS).await;
    let sfx_page = sfx(SFX_PAGE).await;
    let sfx_pot = sfx(SFX_POT).await;
    let sfx_blip = sfx(SFX_BLIP).await;
    let sfx_rune = sfx(SFX_RUNE).await;
    let sfx_stone = sfx(SFX_STONE).await;
    let sfx_rune_cast = sfx(SFX_RUNE_CAST).await;
    let encounter_music = sfx(ENCOUNTER_MUSIC).await;
    let rest_music = sfx(REST_MUSIC).await;

    // The currently-looping overworld track, as (zone index, is_night) — the
    // night flag is part of the identity so dusk and dawn swap the loop even
    // without leaving the zone.
    let mut playing_zone: Option<(usize, bool)> = None;
    let mut playing_theme = false;
    // Which zone's daytime weather bed is looping, if any.
    let mut playing_day_bed: Option<usize> = None;
    let mut playing_title = false;
    let mut playing_hearth = false;
    let mut playing_encounter = false;
    let mut playing_rest = false;
    let mut cue = Cue::None;
    // Foley bookkeeping: footsteps land every *other* step, and the two
    // takes of each surface alternate so a walk never metronomes.
    let mut step_flip = false;
    let mut step_alt = false;
    // The sound option, as a master gain over every loop and one-shot.
    // Re-leveled live when the rest menu's dial moves.
    let mut gain = sound_gain(app.sound_level);
    // When the next owl hoot is due (`mq::get_time()` seconds); `None` while
    // it isn't night, so the first night schedules a fresh call.
    let mut owl_at: Option<f64> = None;

    // The texture is recreated whenever the framebuffer's size changes (a
    // window resize), so it always matches the pixels we're pushing.
    let mut image = mq::Image::gen_image_color(fb.w as u16, fb.h as u16, mq::BLACK);
    let mut texture = mq::Texture2D::from_image(&image);
    texture.set_filter(mq::FilterMode::Nearest);
    let mut fb_dims = (fb.w, fb.h);

    let mut tick_acc = 0.0f32;
    // Every physical key currently held to walk, with the game key it sends.
    // Arrows and vim keys both feed this one mechanism, so they start walking
    // after the same `REPEAT_AFTER` and step at the same `REPEAT_EVERY` — the
    // OS key-repeat settings never enter into it. Holding two keys on
    // different axes walks the diagonal.
    let mut held: Vec<(mq::KeyCode, Key)> = Vec::new();
    let mut walk_t = 0.0f32;
    let mut fullscreen = false;

    while !app.should_quit {
        // Alt+Enter toggles fullscreen — a shell-level window concern that
        // never reaches the game state, so the Enter it rides on is swallowed
        // here instead of being read as a confirm.
        let alt = mq::is_key_down(mq::KeyCode::LeftAlt) || mq::is_key_down(mq::KeyCode::RightAlt);
        let toggle_fullscreen = alt
            && (mq::is_key_pressed(mq::KeyCode::Enter) || mq::is_key_pressed(mq::KeyCode::KpEnter));
        if toggle_fullscreen {
            fullscreen = !fullscreen;
            mq::set_fullscreen(fullscreen);
        }

        // Physical keys newly pressed this frame (never OS auto-repeats —
        // macroquad only records a press when its `repeat` flag is false). This
        // lets a held vim key drive walking off physical key state, the same way
        // the arrows do, so its start delay and pace are ours rather than the
        // OS's.
        let pressed = mq::get_keys_pressed();

        // Character keys come from the OS text stream, so they respect the
        // active keyboard layout (Dvorak, AZERTY, …) instead of raw QWERTY
        // positions. This drives the vim movement keys, the command letters,
        // and typing a name.
        while let Some(c) = mq::get_char_pressed() {
            let c = c.to_ascii_lowercase();
            // A held vim key streams repeated chars at the OS rate; ignore those
            // and let the timer below walk instead. Only a genuine new press
            // (one that shows up in `pressed`) starts a walk — binding the
            // physical key that produced it so the repeat tracks the real hold
            // whatever the layout.
            if matches!(c, 'h' | 'j' | 'k' | 'l') && matches!(app.screen, Screen::World) {
                if let Some(&mk) = pressed.iter().find(|k| !MOVEMENT.contains(*k)) {
                    // A fresh press from standstill steps right away; a key
                    // joining an ongoing walk waits for the next scheduled
                    // repeat instead. An immediate step mid-glide would land
                    // while the previous one is still being drawn and snap
                    // the traveller (and camera) half a tile — the diagonal
                    // "hiccup" of playtest fame.
                    if held.is_empty() {
                        app.on_key(Key::Char(c));
                        held.push((mk, Key::Char(c)));
                        walk_t = 0.0;
                    } else if !held.iter().any(|&(k, _)| k == mk) {
                        held.push((mk, Key::Char(c)));
                    }
                }
            } else if c.is_ascii_alphabetic() || c == '-' || c == '\'' {
                app.on_key(Key::Char(c));
            }
        }
        // Discrete presses of the non-character keys go straight to on_key —
        // except an arrow joining an ongoing walk, which (like the vim keys
        // above) waits for the next repeat so the step-glide never snaps.
        for &(mk, code) in KEYMAP {
            if mq::is_key_pressed(mk) {
                let walking = MOVEMENT.contains(&mk) && matches!(app.screen, Screen::World);
                if walking && !held.is_empty() {
                    if !held.iter().any(|&(k, _)| k == mk) {
                        held.push((mk, code));
                    }
                    continue;
                }
                app.on_key(code);
                if walking && !held.iter().any(|&(k, _)| k == mk) {
                    held.push((mk, code));
                    walk_t = 0.0;
                }
            }
        }
        // Hold-to-walk: repeat the held movement keys while any stay down.
        // One key walks a line; keys on both axes walk the diagonal, the
        // newest press deciding which way the traveller faces.
        held.retain(|&(mk, _)| mq::is_key_down(mk));
        if held.is_empty() || !matches!(app.screen, Screen::World) {
            walk_t = 0.0;
        } else {
            // The latest-held key per axis, oldest axis stepping first so
            // the most recent press wins the facing.
            let mut moves: Vec<Key> = Vec::new();
            for &(_, key) in &held {
                if let Some(d) = dir_of(key) {
                    moves.retain(|k| dir_of(*k).is_none_or(|e| (e.0 != 0) != (d.0 != 0)));
                    moves.push(key);
                }
            }
            let step = if moves.len() > 1 {
                REPEAT_EVERY * DIAGONAL_STRETCH
            } else {
                REPEAT_EVERY
            };
            walk_t += mq::get_frame_time();
            if walk_t >= REPEAT_AFTER {
                walk_t -= step;
                for &key in &moves {
                    // The first step may open a door, a gate or an encounter;
                    // the second only lands if we're still on the road.
                    if matches!(app.screen, Screen::World) {
                        app.on_key(key);
                    }
                }
            }
        }

        tick_acc += mq::get_frame_time();
        while tick_acc >= TICK_SECS {
            tick_acc -= TICK_SECS;
            app.on_tick();
        }
        // Where we are inside the current tick, for the renderer's step-glide.
        app.subtick = (tick_acc / TICK_SECS).clamp(0.0, 1.0);

        // The sound dial (rest menu): when it turns, re-level every live
        // loop so the change lands mid-note instead of at the next song.
        let next_gain = sound_gain(app.sound_level);
        if next_gain != gain {
            gain = next_gain;
            if playing_title {
                audio::set_sound_volume(&title_music, MUSIC_VOLUME * gain);
            }
            if let Some((z, night)) = playing_zone {
                let (sound, volume) = if night {
                    (&night_music[z], NIGHT_VOLUME)
                } else {
                    (&zone_music[z], DAY_MUSIC_VOLUME)
                };
                audio::set_sound_volume(sound, volume * gain);
            }
            if playing_theme {
                audio::set_sound_volume(&night_theme, NIGHT_THEME_VOLUME * gain);
            }
            if let Some(z) = playing_day_bed
                && let Some(bed) = &day_beds[z]
            {
                audio::set_sound_volume(bed, DAY_BED_VOLUME * gain);
            }
            if playing_hearth {
                audio::set_sound_volume(&hearth_loop, HEARTH_VOLUME * gain);
            }
            if playing_encounter {
                audio::set_sound_volume(&encounter_music, ENCOUNTER_VOLUME * gain);
            }
            if playing_rest {
                audio::set_sound_volume(&rest_music, REST_VOLUME * gain);
            }
        }

        // The lib narrates, the shell speaks: every queued sound event
        // becomes audio here and nowhere else. Drained even with the dial
        // off, so nothing piles up.
        for event in app.drain_sounds() {
            let played = match event {
                SoundEvent::Stepped(terrain) => {
                    step_flip = !step_flip;
                    if !step_flip {
                        continue; // quiet feet: every other step sounds
                    }
                    step_alt = !step_alt;
                    steps
                        .iter()
                        .find(|(t, _)| *t == terrain)
                        .map(|(_, takes)| (&takes[step_alt as usize], STEP_VOLUME))
                }
                SoundEvent::DoorUsed => Some((&sfx_door, FOLEY_VOLUME)),
                SoundEvent::ChestOpened => Some((&sfx_chest, FOLEY_VOLUME)),
                SoundEvent::KeepsakeGiven => Some((&sfx_coins, FOLEY_VOLUME)),
                SoundEvent::CoinsTraded => Some((&sfx_coins, FOLEY_VOLUME)),
                SoundEvent::DishCooked => Some((&sfx_pot, FOLEY_VOLUME)),
                SoundEvent::PageTurned => Some((&sfx_page, 0.4)),
                SoundEvent::MenuMoved => Some((&sfx_blip, BLIP_VOLUME)),
                SoundEvent::RuneCaught => Some((&sfx_rune, JINGLE_VOLUME)),
                SoundEvent::StoneFound => Some((&sfx_stone, JINGLE_VOLUME)),
                SoundEvent::RuneCast => Some((&sfx_rune_cast, JINGLE_VOLUME)),
            };
            if let Some((sound, volume)) = played
                && gain > 0.0
            {
                audio::play_sound(
                    sound,
                    PlaySoundParams {
                        looped: false,
                        volume: volume * gain,
                    },
                );
            }
        }

        // Title theme: loops through the title and char-select screens, then
        // gives way to zone music once the journey actually starts.
        let in_menus = matches!(app.screen, Screen::Title { .. } | Screen::CharSelect { .. });
        if in_menus != playing_title {
            if in_menus {
                audio::play_sound(
                    &title_music,
                    PlaySoundParams {
                        looped: true,
                        volume: MUSIC_VOLUME * gain,
                    },
                );
            } else {
                audio::stop_sound(&title_music);
            }
            playing_title = in_menus;
        }

        // Zone music: one loop per overworld region, swapped on warp — and
        // after dark, swapped for that region's calmer night ambience.
        // Interiors and the title/char-select screens stay quiet. The night
        // flag rides in the track identity so the loop also swaps when the
        // day/night clock turns, without any zone change.
        let past_menus = !in_menus;
        let night = app.is_night();
        let zone_track = rgame::world::zones::region_of(app.zone_idx)
            .filter(|&r| past_menus && r < zone_music.len())
            .map(|r| (r, night));
        if zone_track != playing_zone {
            let track = |&(z, on): &(usize, bool)| -> (&Sound, f32) {
                if on {
                    (&night_music[z], NIGHT_VOLUME)
                } else {
                    (&zone_music[z], DAY_MUSIC_VOLUME)
                }
            };
            if let Some(old) = playing_zone.as_ref() {
                audio::stop_sound(track(old).0);
            }
            if let Some(new) = zone_track.as_ref() {
                let (sound, volume) = track(new);
                audio::play_sound(
                    sound,
                    PlaySoundParams {
                        looped: true,
                        volume: volume * gain,
                    },
                );
            }
            playing_zone = zone_track;
        }

        // Indoors, the fire keeps you company: the lived-in rooms loop a soft
        // hearth crackle where the zone music leaves off. Caves and bare
        // storerooms stay properly silent.
        let want_hearth = past_menus && rgame::world::zones::has_hearth(app.zone_idx);
        if want_hearth != playing_hearth {
            if want_hearth {
                audio::play_sound(
                    &hearth_loop,
                    PlaySoundParams {
                        looped: true,
                        volume: HEARTH_VOLUME * gain,
                    },
                );
            } else {
                audio::stop_sound(&hearth_loop);
            }
            playing_hearth = want_hearth;
        }

        // The night theme rides over whichever nature bed is playing: one
        // calm melody for the whole night, started at dusk (or on stepping
        // outside after dark) and stopped at dawn or back indoors.
        let want_theme = matches!(zone_track, Some((_, true)));
        if want_theme != playing_theme {
            if want_theme {
                audio::play_sound(
                    &night_theme,
                    PlaySoundParams {
                        looped: true,
                        volume: NIGHT_THEME_VOLUME * gain,
                    },
                );
            } else {
                audio::stop_sound(&night_theme);
            }
            playing_theme = want_theme;
        }

        // By day, a zone whose drawn weather makes a sound lays it under the
        // chiptune: wind through the Woods' canopy, Silverford's rain, wind
        // off the Hearthspire road. Swapped on warp like the zone music, and
        // it yields to the night beds after dark (which carry their own
        // weather already).
        let day_bed_track = match zone_track {
            Some((z, false)) if day_beds[z].is_some() => Some(z),
            _ => None,
        };
        if day_bed_track != playing_day_bed {
            if let Some(z) = playing_day_bed
                && let Some(bed) = &day_beds[z]
            {
                audio::stop_sound(bed);
            }
            if let Some(z) = day_bed_track
                && let Some(bed) = &day_beds[z]
            {
                audio::play_sound(
                    bed,
                    PlaySoundParams {
                        looped: true,
                        volume: DAY_BED_VOLUME * gain,
                    },
                );
            }
            playing_day_bed = day_bed_track;
        }

        // A wild rune stirring gets its sting: an urgent little chiptune
        // looped under the whole encounter, gone the moment it resolves.
        let want_encounter = matches!(app.screen, Screen::Encounter { .. });
        if want_encounter != playing_encounter {
            if want_encounter {
                audio::play_sound(
                    &encounter_music,
                    PlaySoundParams {
                        looped: true,
                        volume: ENCOUNTER_VOLUME * gain,
                    },
                );
            } else {
                audio::stop_sound(&encounter_music);
            }
            playing_encounter = want_encounter;
        }

        // And the campfire gets its rest theme, playing softly for as long
        // as the embers hold the screen.
        let want_rest = matches!(app.screen, Screen::Resting { .. });
        if want_rest != playing_rest {
            if want_rest {
                audio::play_sound(
                    &rest_music,
                    PlaySoundParams {
                        looped: true,
                        volume: REST_VOLUME * gain,
                    },
                );
            } else {
                audio::stop_sound(&rest_music);
            }
            playing_rest = want_rest;
        }

        // A distant owl, at night only: a soft one-shot fired at randomized
        // gaps so it never falls into a rhythm. Rescheduled off `mq::get_time`
        // each time it sounds; cleared the moment it stops being night so the
        // next nightfall opens with a fresh, unhurried wait.
        if night {
            let now = mq::get_time();
            match owl_at {
                None => owl_at = Some(now + macroquad::rand::gen_range(6.0, 22.0)),
                Some(due) if now >= due => {
                    audio::play_sound(
                        &sfx_owl,
                        PlaySoundParams {
                            looped: false,
                            volume: OWL_VOLUME * gain,
                        },
                    );
                    owl_at = Some(now + macroquad::rand::gen_range(20.0, 55.0));
                }
                _ => {}
            }
        } else {
            owl_at = None;
        }

        // Cast/pass/fizzle SFX: one-shot, fired on the frame the screen turns
        // into that cue (never re-fired while it holds).
        let next_cue = cue_for(&app.screen);
        if next_cue != cue {
            let sound = match next_cue {
                Cue::Casting => Some(&sfx_cast),
                Cue::Pass => Some(&sfx_pass),
                Cue::Fizzle => Some(&sfx_fizzle),
                Cue::None => None,
            };
            if let Some(sound) = sound {
                audio::play_sound(
                    sound,
                    PlaySoundParams {
                        looped: false,
                        volume: SFX_VOLUME * gain,
                    },
                );
            }
            cue = next_cue;
        }

        // Size the framebuffer to the window so the picture fills it entirely —
        // no black bars, and wide screens reveal more of the world.
        let (sw, sh) = (mq::screen_width(), mq::screen_height());
        let (scale, fb_w, fb_h) = fit(sw, sh);
        fb.resize(fb_w, fb_h);
        if fb_dims != (fb.w, fb.h) {
            image = mq::Image::gen_image_color(fb.w as u16, fb.h as u16, mq::BLACK);
            texture = mq::Texture2D::from_image(&image);
            texture.set_filter(mq::FilterMode::Nearest);
            fb_dims = (fb.w, fb.h);
        }

        gfx::render(&mut fb, &atlas, &app);
        image.bytes.copy_from_slice(&fb.px);
        texture.update(&image);

        // Crisp integer scale, centered — the framebuffer is already sized to
        // cover the window, so any overhang is a symmetric pixel or two.
        let (dw, dh) = (fb.w as f32 * scale as f32, fb.h as f32 * scale as f32);
        mq::clear_background(mq::BLACK);
        mq::draw_texture_ex(
            &texture,
            (sw - dw) / 2.0,
            (sh - dh) / 2.0,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::vec2(dw, dh)),
                ..Default::default()
            },
        );
        mq::next_frame().await;
    }
}
