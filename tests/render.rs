//! Renders every screen at a spread of terminal sizes — tiny, classic 80x24,
//! wide, and ultrawide-bigger-than-the-map — to prove nothing panics and the
//! camera/out-of-bounds fill hold up everywhere.

use ratatui::Terminal;
use ratatui::backend::TestBackend;

use rgame::app::{App, Dialogue, DialogueKind, Screen};
use rgame::checker::Outcome;
use rgame::ui;
use rgame::world::map::{MAP_H, MAP_W};

const SIZES: &[(u16, u16)] = &[(10, 5), (80, 24), (160, 45), (300, 90)];

fn render(app: &App, w: u16, h: u16) {
    let backend = TestBackend::new(w, h);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| ui::draw(frame, app)).unwrap();
}

fn render_all_sizes(app: &App) {
    for &(w, h) in SIZES {
        render(app, w, h);
    }
}

#[test]
fn every_screen_renders_at_every_size() {
    let mut app = App::new();
    app.tick = 12345; // some arbitrary time of day, animations mid-frame

    render_all_sizes(&app); // Title

    for zone in 0..4 {
        app.zone_idx = zone;
        app.player = app.zones[zone].spawn;
        app.screen = Screen::World;
        render_all_sizes(&app);
    }

    // Camera pinned to the map corners, and past them.
    app.zone_idx = 0;
    for corner in [
        (0, 0),
        (MAP_W - 1, MAP_H - 1),
        (MAP_W - 1, 0),
        (0, MAP_H - 1),
    ] {
        app.player = corner;
        render_all_sizes(&app);
    }

    app.player = app.zones[0].spawn;
    app.screen = Screen::Dialogue(Dialogue {
        speaker: "Elder Rowan".to_string(),
        pages: vec![
            "Hello there, traveler!".to_string(),
            "Second page.".to_string(),
        ],
        page: 0,
        revealed: 8,
        kind: DialogueKind::Flavor,
    });
    render_all_sizes(&app);

    app.screen = Screen::Journal;
    render_all_sizes(&app);
    app.accepted.insert(1);
    app.hints.insert(1, 2);
    app.screen = Screen::Journal;
    render_all_sizes(&app);

    app.screen = Screen::Casting { quest: 1 };
    render_all_sizes(&app);

    for outcome in [
        Outcome::Pass {
            output: "ok".to_string(),
        },
        Outcome::CompileFail {
            stderr: "error[E0423]: expected function\n --> line 21".repeat(30),
        },
        Outcome::TestFail {
            output: "test the_ledger_adds_up ... FAILED".to_string(),
        },
        Outcome::Timeout,
        Outcome::Error {
            msg: "rustc missing".to_string(),
        },
    ] {
        app.screen = Screen::CastResult {
            quest: 1,
            outcome,
            scroll: 3,
        };
        render_all_sizes(&app);
    }

    app.screen = Screen::Paused { selected: 1 };
    render_all_sizes(&app);

    for page in 0..4 {
        app.screen = Screen::Epilogue { page };
        render_all_sizes(&app);
    }
}

#[test]
fn a_completed_game_still_renders() {
    let mut app = App::new();
    app.completed.extend(1..=12u8);
    app.accepted.extend(1..=12u8);
    app.zone_idx = 3;
    app.player = app.zones[3].spawn;
    app.screen = Screen::World;
    app.tick = 99999;
    render_all_sizes(&app);
    app.screen = Screen::Journal;
    render_all_sizes(&app);
}
