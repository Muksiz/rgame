//! Render any game screen to a PNG without a window. The framebuffer here is
//! byte-identical to what the windowed game displays, so this is how to "see"
//! the game headless — map edits, new screens, all of it.
//!
//! ```sh
//! cargo run --example snapshot -- world 0 --tick 600 --out shot.png
//! cargo run --example snapshot -- <title|charselect|world|dialogue|journal|casting|pass|fizzle|paused|resting|banner|epilogue|toast|encounter|caught|grimoire|book|reveal>
//! ```
//!
//! `world` takes an optional zone (0-3 overworld, 4+ interiors) and
//! `--pos x,y`; `--tick` drives animations; `--day <ticks>` sets the position
//! in the day/night clock (0 = dawn; see `rgame::app::DAY_LEN`), so outdoor
//! scenes can be shot at any hour; `--crab` places Ferris one tile west of
//! the player (headless shots otherwise leave him tucked under the player,
//! where he isn't drawn). Default output: snapshot.png.

use std::io::BufWriter;

use rgame::app::{App, Dialogue, DialogueKind, EncounterPhase, Screen};
use rgame::checker::Outcome;
use rgame::content::quests::QUESTS;
use rgame::gfx::{self, Atlas, Frame};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let scene = args.first().map(String::as_str).unwrap_or("world");
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1).cloned())
    };
    let tick: u64 = flag("--tick").and_then(|s| s.parse().ok()).unwrap_or(600);
    let out = flag("--out").unwrap_or_else(|| "snapshot.png".into());

    let mut app = App::new();
    app.tick = tick;
    app.day_ticks = flag("--day")
        .and_then(|s| s.parse().ok())
        .map(|d: u32| d % rgame::app::DAY_LEN)
        .unwrap_or(0);
    app.screen = Screen::World;

    // `world 2` etc: jump to a zone with earlier quests completed.
    if let Some(z) = args.get(1).and_then(|s| s.parse::<usize>().ok()) {
        app.zone_idx = z.min(app.zones.len() - 1);
        app.player = app.zones[app.zone_idx].spawn;
        for q in QUESTS.iter().filter(|q| q.zone < app.zone_idx) {
            app.completed.insert(q.id);
        }
    }
    if let Some(pos) = flag("--pos") {
        let (x, y) = pos.split_once(',').expect("--pos x,y");
        app.player = (x.parse().unwrap(), y.parse().unwrap());
    }
    // `--crab`: Ferris visible at heel instead of tucked under the player.
    if args.iter().any(|a| a == "--crab") {
        app.companion = (app.player.0 - 1, app.player.1);
        app.companion_prev = app.companion;
    }

    match scene {
        "world" => {}
        "title" => app.screen = Screen::Title { selected: 0 },
        "charselect" => {
            app.screen = Screen::CharSelect {
                idx: (tick as usize) % 4,
                name: String::new(),
            }
        }
        "toast" => app.toast("A quiet morning in Emberwick. Someone near the festival square could use a hand. (Arrows or H J K L to walk, e to talk.)"),
        "dialogue" => {
            let q = app.active_quest().expect("road not finished");
            let pages = q.intro.iter().map(|p| p.to_string()).collect();
            let mut d = Dialogue {
                speaker: q.npc.to_string(),
                pages,
                page: 0,
                revealed: 0,
                kind: DialogueKind::Intro(q.id),
            };
            d.revealed = 220;
            app.screen = Screen::Dialogue(d);
        }
        "journal" => {
            let q = app.active_quest().expect("road not finished");
            app.accepted.insert(q.id);
            app.hints.insert(q.id, 2);
            app.screen = Screen::Journal;
        }
        "casting" => app.screen = Screen::Casting { quest: 1 },
        "pass" => {
            app.screen = Screen::CastResult {
                quest: 1,
                outcome: Outcome::Pass { output: String::new() },
                scroll: 0,
            }
        }
        "fizzle" => {
            let stderr = "error[E0308]: mismatched types\n --> quests/01_the_unlit_lantern.rs:12:20\n   |\n12 |     let lit: bool = \"yes\";\n   |              ----   ^^^^^ expected `bool`, found `&str`\n   |              |\n   |              expected due to this\n\nerror: aborting due to 1 previous error\nFor more information about this error, try `rustc --explain E0308`.";
            app.screen = Screen::CastResult {
                quest: 1,
                outcome: Outcome::CompileFail { stderr: stderr.into() },
                scroll: 0,
            }
        }
        "paused" => app.screen = Screen::Paused { selected: 1 },
        "banner" => {
            let name = app.zone().name.to_string();
            app.banner = Some((name, app.tick + 40));
        }
        "resting" => {
            app.screen = Screen::Resting {
                lore: (tick as usize) % rgame::content::lore::LORE.len(),
                t: 40,
                wake: rgame::app::DayPhase::Night,
            }
        }
        "epilogue" => app.screen = Screen::Epilogue { page: 1 },
        "encounter" => {
            app.screen = Screen::Encounter {
                rune: 11, // the legendary Turbofish
                selected: 1,
                phase: EncounterPhase::Asking,
            }
        }
        "caught" => {
            app.grimoire.insert(11);
            app.screen = Screen::Encounter {
                rune: 11,
                selected: 1,
                phase: EncounterPhase::Caught,
            }
        }
        "grimoire" => {
            app.grimoire.extend([1, 2, 5, 11]);
            app.screen = Screen::Grimoire;
        }
        // The gate-reveal cutscene: the zone's quests done, the camera out
        // at the gate, the barrier rolling aside. The reveal starts at tick
        // 600, so `--tick 600..=660` scrubs through its phases (the default
        // 600 is the first frame; ~625 is mid-clear).
        "reveal" => {
            for q in QUESTS.iter().filter(|q| q.zone == app.zone_idx) {
                app.completed.insert(q.id);
            }
            app.gate_reveal = Some((app.zone_idx, tick.min(600)));
        }
        "book" => {
            let book = &rgame::content::books::BOOKS[1];
            app.screen = Screen::Dialogue(Dialogue {
                speaker: book.title.to_string(),
                pages: book.pages.iter().map(|p| p.to_string()).collect(),
                page: 0,
                revealed: 500,
                kind: DialogueKind::Book,
            });
        }
        other => {
            eprintln!("unknown scene: {other}");
            std::process::exit(2);
        }
    }

    // The folk keep their schedule in shots too: `--day` past nightfall
    // finds everyone at their night spots (57000 is the deep of night).
    app.apply_schedule();

    let atlas = Atlas::load();
    // `--size WxH` renders at a non-native framebuffer size (e.g. 960x270 for a
    // 32:9 superultrawide) to preview how the window fills without black bars.
    let mut fb = match flag("--size") {
        Some(s) => {
            let (w, h) = s.split_once('x').expect("--size WxH");
            Frame::with_size(w.parse().unwrap(), h.parse().unwrap())
        }
        None => Frame::new(),
    };
    gfx::render(&mut fb, &atlas, &app);

    let file = std::fs::File::create(&out).expect("create output png");
    let mut enc = png::Encoder::new(BufWriter::new(file), fb.w as u32, fb.h as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().expect("png header");
    writer.write_image_data(&fb.px).expect("png data");
    println!(
        "wrote {out} ({}x{}, scene: {scene}, tick: {tick})",
        fb.w, fb.h
    );
}
