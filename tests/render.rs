//! Renders every game screen through the real renderer into the 480×270
//! framebuffer — every zone, the camera pinned to the map corners and past
//! them — to prove nothing panics and the out-of-bounds fill holds up.

use rgame::app::{App, Dialogue, DialogueKind, EncounterPhase, Screen};
use rgame::checker::Outcome;
use rgame::content::books;
use rgame::gfx::{self, Atlas, Frame};
use rgame::world::map::{MAP_H, MAP_W};

fn render(atlas: &Atlas, app: &App) {
    let mut fb = Frame::new();
    gfx::render(&mut fb, atlas, app);
}

#[test]
fn every_screen_renders() {
    let atlas = Atlas::load();
    let mut app = App::new();
    app.tick = 12345; // animations mid-frame

    render(&atlas, &app); // Title

    // The character chooser, empty and with a typed name.
    app.screen = Screen::CharSelect {
        idx: 0,
        name: String::new(),
    };
    render(&atlas, &app);
    app.screen = Screen::CharSelect {
        idx: 3,
        name: "Bramble".to_string(),
    };
    render(&atlas, &app);
    app.screen = Screen::Title { selected: 0 };

    // Every zone, interiors included — each keeps its own hour and weather.
    for zone in 0..app.zones.len() {
        app.zone_idx = zone;
        app.player = app.zones[zone].spawn;
        app.screen = Screen::World;
        render(&atlas, &app);
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
        render(&atlas, &app);
    }

    app.player = app.zones[0].spawn;
    app.toast("A toast! To toast.");
    render(&atlas, &app);

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
    render(&atlas, &app);

    // A book off a Library shelf gets its own portrait.
    let book = &books::BOOKS[0];
    app.screen = Screen::Dialogue(Dialogue {
        speaker: book.title.to_string(),
        pages: book.pages.iter().map(|p| p.to_string()).collect(),
        page: 0,
        revealed: 400,
        kind: DialogueKind::Book,
    });
    render(&atlas, &app);

    app.screen = Screen::Journal;
    render(&atlas, &app);
    app.accepted.insert(1);
    app.hints.insert(1, 2);
    app.completed.extend([1, 2, 3, 8]); // satchel has keepsakes to show
    app.fish = 3;
    // Side business underway: trinkets carried, notes pending, stones found.
    app.set_flag(rgame::content::sides::SORREL_ASKED);
    app.set_flag(rgame::content::sides::SORREL_MINT);
    app.set_flag(rgame::content::sides::NETTLE_MET);
    app.set_flag(&rgame::content::sides::runestone_flag(1));
    app.set_flag(&rgame::content::sides::runestone_flag(8));
    app.screen = Screen::Journal;
    render(&atlas, &app);

    // A runestone reading itself aloud, stone portrait and all.
    let stone = rgame::content::stones::stone(1);
    app.screen = Screen::Dialogue(Dialogue {
        speaker: stone.name.to_string(),
        pages: vec![stone.legend.to_string()],
        page: 0,
        revealed: 400,
        kind: DialogueKind::Stone,
    });
    render(&atlas, &app);

    for phase in [
        EncounterPhase::Asking,
        EncounterPhase::Caught,
        EncounterPhase::Fizzled,
    ] {
        app.screen = Screen::Encounter {
            rune: 11,
            selected: 1,
            phase,
        };
        render(&atlas, &app);
    }

    app.grimoire.extend([1, 5, 11]);
    app.screen = Screen::Grimoire;
    render(&atlas, &app);

    app.screen = Screen::Casting { quest: 1 };
    render(&atlas, &app);

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
        render(&atlas, &app);
    }

    app.screen = Screen::Paused { selected: 1 };
    render(&atlas, &app);

    // A zone-arrival banner, mid-slide.
    app.screen = Screen::World;
    app.banner = Some(("Whispering Woods".to_string(), app.tick + 30));
    render(&atlas, &app);
    app.banner = None;

    // Resting by a campfire, mid fade-in.
    app.screen = Screen::Resting {
        lore: 3,
        t: 2,
        wake: rgame::app::DayPhase::Night,
    };
    render(&atlas, &app);
    app.screen = Screen::Resting {
        lore: 10,
        t: 40,
        wake: rgame::app::DayPhase::Morning,
    };
    render(&atlas, &app);

    // The overworld at night: NPCs asleep, moon in the HUD.
    app.day_ticks = rgame::app::DAY_LEN - 5000;
    app.zone_idx = 0;
    app.player = app.zones[0].spawn;
    app.screen = Screen::World;
    render(&atlas, &app);
    app.day_ticks = 0;

    for page in 0..4 {
        app.screen = Screen::Epilogue { page };
        render(&atlas, &app);
    }
}

#[test]
fn a_completed_game_still_renders() {
    let atlas = Atlas::load();
    let mut app = App::new();
    app.completed.extend(1..=12u8);
    app.accepted.extend(1..=12u8);
    app.grimoire.extend(1..=16u8);
    for id in 1..=8 {
        app.set_flag(&rgame::content::sides::runestone_flag(id));
    }
    app.set_flag(rgame::content::sides::SORREL_DONE);
    app.set_flag(rgame::content::sides::CHEST_OPENED);
    app.zone_idx = 3;
    app.player = app.zones[3].spawn;
    app.screen = Screen::World;
    app.tick = 99999;
    render(&atlas, &app);
    app.screen = Screen::Journal;
    render(&atlas, &app);
    app.screen = Screen::Grimoire;
    render(&atlas, &app);
}
