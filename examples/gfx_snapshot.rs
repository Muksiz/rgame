//! Render any graphical screen to a PNG without a window — the gfx sibling of
//! the `snapshot` example. The framebuffer here is byte-identical to what the
//! Macroquad frontend displays, so this is how to "see" the game headless.
//!
//! ```sh
//! cargo run --example gfx_snapshot -- world 0 --tick 600 --out shot.png
//! cargo run --example gfx_snapshot -- <title|world|dialogue|journal|casting|pass|fizzle|paused|epilogue|toast>
//! ```
//!
//! `world` takes an optional zone (0-3) and `--pos x,y`; `--tick` sets time of
//! day (600 ≈ noon, 3000 ≈ night). Default output: gfx_snapshot.png.

use std::io::BufWriter;

use rgame::app::{App, Dialogue, DialogueKind, Screen};
use rgame::checker::Outcome;
use rgame::content::quests::QUESTS;
use rgame::gfx::{self, Atlas, FB_H, FB_W, Frame};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let scene = args.first().map(String::as_str).unwrap_or("world");
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1).cloned())
    };
    let tick: u64 = flag("--tick").and_then(|s| s.parse().ok()).unwrap_or(600);
    let out = flag("--out").unwrap_or_else(|| "gfx_snapshot.png".into());

    let mut app = App::new();
    app.tick = tick;
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

    match scene {
        "world" => {}
        "title" => app.screen = Screen::Title { selected: 0 },
        "toast" => app.toast("A quiet morning in Emberwick. Someone near the festival square could use a hand. (Arrows/WASD to walk, e to talk.)"),
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
        "paused" => app.screen = Screen::Paused { selected: 0 },
        "epilogue" => app.screen = Screen::Epilogue { page: 1 },
        other => {
            eprintln!("unknown scene: {other}");
            std::process::exit(2);
        }
    }

    let atlas = Atlas::load();
    let mut fb = Frame::new();
    gfx::render(&mut fb, &atlas, &app);

    let file = std::fs::File::create(&out).expect("create output png");
    let mut enc = png::Encoder::new(BufWriter::new(file), FB_W as u32, FB_H as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().expect("png header");
    writer.write_image_data(&fb.px).expect("png data");
    println!("wrote {out} ({FB_W}x{FB_H}, scene: {scene}, tick: {tick})");
}
