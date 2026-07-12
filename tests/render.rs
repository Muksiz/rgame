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

    app.grimoire.extend([1, 5, 11, 18]);
    for page in 0..rgame::content::wilds::GRIMOIRE_PAGES {
        app.screen = Screen::Grimoire { page };
        render(&atlas, &app);
    }

    // The trading post: an empty basket (stock only), then a full one with
    // a purse — selected row first and last.
    app.screen = Screen::Trade { selected: 0 };
    render(&atlas, &app);
    app.coins = 12;
    app.pantry.insert(rgame::content::market::Good::Mushroom, 3);
    app.pantry.insert(rgame::content::market::Good::Berries, 1);
    let rows = rgame::content::market::trade_rows(&app.pantry);
    app.screen = Screen::Trade {
        selected: rows.len() - 1,
    };
    render(&atlas, &app);
    app.screen = Screen::Journal; // the purse & basket line
    render(&atlas, &app);
    app.coins = 0;
    app.pantry.clear();

    // The market garden at every stage: seedling, growing, and both crops
    // ripe — camera parked on the walkway between the rows.
    use rgame::content::market::Good;
    app.garden.insert((0, (84, 41)), (Good::Turnip, 0));
    app.garden.insert((0, (85, 41)), (Good::Turnip, 1));
    app.garden.insert((0, (86, 41)), (Good::Turnip, 2));
    app.garden.insert((0, (84, 43)), (Good::Pumpkin, 3));
    app.player = (86, 42);
    app.screen = Screen::World;
    render(&atlas, &app);
    app.screen = Screen::Journal; // the garden tally line
    render(&atlas, &app);
    // The planting chooser, with one seed kind and with both.
    app.pantry.insert(Good::TurnipSeeds, 2);
    app.screen = Screen::Planting {
        at: (85, 43),
        selected: 0,
    };
    render(&atlas, &app);
    app.pantry.insert(Good::PumpkinSeeds, 1);
    app.screen = Screen::Planting {
        at: (85, 43),
        selected: 1,
    };
    render(&atlas, &app);
    app.pantry.clear();
    app.garden.clear();
    app.player = app.zones[0].spawn;

    // Poppy's ovens: the recipe book bare-basketed and stocked, and a
    // favorite dish changing hands.
    app.screen = Screen::Cooking { selected: 0 };
    render(&atlas, &app);
    app.pantry.insert(Good::Mushroom, 2);
    app.pantry.insert(Good::Turnip, 1);
    app.screen = Screen::Cooking { selected: 2 };
    render(&atlas, &app);
    app.pantry.clear();
    let fav = &rgame::content::kitchen::FAVORITES[0];
    app.screen = Screen::Dialogue(Dialogue {
        speaker: fav.npc.to_string(),
        pages: fav.pages.iter().map(|p| p.to_string()).collect(),
        page: 0,
        revealed: 400,
        kind: DialogueKind::Gift {
            flag: fav.flag,
            dish: fav.dish,
        },
    });
    render(&atlas, &app);

    // The casting ring: standing quiet, part-filled, and brimming.
    let stashed = std::mem::take(&mut app.grimoire);
    app.screen = Screen::RuneRing { selected: 0 };
    render(&atlas, &app);
    app.grimoire = stashed;
    app.screen = Screen::RuneRing { selected: 2 };
    render(&atlas, &app);
    app.grimoire
        .extend(rgame::content::wilds::WILDS.iter().map(|w| w.id));
    app.screen = Screen::RuneRing {
        selected: rgame::content::wilds::WILDS.len() - 1,
    };
    render(&atlas, &app);

    // Every cast shape mid-flight over the world, early and late in the arc
    // (one rune per shape suffices — the shape is the renderer's whole key).
    app.screen = Screen::World;
    let mut seen = Vec::new();
    for w in &rgame::content::wilds::WILDS {
        let shape = rgame::content::casts::cast(w.id).shape;
        if seen.contains(&shape) {
            continue;
        }
        seen.push(shape);
        for burn in [2, 20, 42] {
            app.rune_fx = Some(rgame::app::RuneFx {
                rune: w.id,
                at: app.player,
                start: app.tick.saturating_sub(burn),
                seek: None,
            });
            render(&atlas, &app);
        }
    }
    // The seeking cast with a stone to lean toward.
    app.rune_fx = Some(rgame::app::RuneFx {
        rune: 16,
        at: app.player,
        start: app.tick.saturating_sub(20),
        seek: Some((app.player.0 + 12, app.player.1 - 4)),
    });
    render(&atlas, &app);
    app.rune_fx = None;

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

    // The parchment map, in every combination of charted regions — and once
    // from a room behind a door, where the dot resolves to the overworld.
    let regions = rgame::world::zones::REGIONS;
    app.screen = Screen::WorldMap;
    for mask in 0..(1u32 << regions.len()) {
        app.flags.retain(|f| !f.starts_with("visited."));
        for (i, &z) in regions.iter().enumerate() {
            if mask & (1 << i) != 0 {
                app.flags.insert(rgame::content::sides::visited_flag(z));
            }
        }
        render(&atlas, &app);
    }
    app.zone_idx = 10; // the Great Library: an interior, two doors deep of nothing
    render(&atlas, &app);
    app.zone_idx = 0;
    app.flags.clear();
    app.screen = Screen::World;

    // The gate-reveal cutscene: Emberwick cleared, the camera out at the
    // gate, the fallen oak mid-roll — then the road standing open after.
    app.zone_idx = 0;
    app.player = app.zones[0].spawn;
    for id in 1..=7 {
        app.completed.insert(id);
    }
    app.gate_reveal = Some((0, app.tick.saturating_sub(20)));
    render(&atlas, &app);
    app.gate_reveal = None;
    render(&atlas, &app); // cleared gates simply stand open
    app.completed.clear();

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

    // The companion at your heels: Ferris, with you from the first morning.
    let spawn = app.zones[0].spawn;
    app.player = spawn;
    app.companion = (spawn.0 - 1, spawn.1);
    app.companion_prev = (spawn.0 - 2, spawn.1);
    render(&atlas, &app); // sitting at your heels
    app.walked_at = app.tick; // mid-stride: the scuttle frames
    render(&atlas, &app);
    app.walked_at = 0;
    app.day_ticks = rgame::app::DAY_LEN - 5000;
    render(&atlas, &app); // dozing at night, z and all
    app.day_ticks = 0;
    // Peeking out of the tall grass, eyestalks only.
    let grass = (0..MAP_H)
        .flat_map(|y| (0..MAP_W).map(move |x| (x, y)))
        .find(|&(x, y)| {
            app.zones[0].tile(x, y) == rgame::world::map::Tile::TallGrass
                && app.zones[0].tile(x + 1, y).walkable()
        })
        .expect("Emberwick grows tall grass beside a path");
    app.player = (grass.0 + 1, grass.1);
    app.companion = grass;
    app.companion_prev = grass;
    render(&atlas, &app);
    // A startled hop when a wild rune stirs.
    app.screen = Screen::Encounter {
        rune: 1,
        selected: 0,
        phase: EncounterPhase::Asking,
    };
    render(&atlas, &app);
    // And curled in the ember-light at a campfire rest.
    app.screen = Screen::Resting {
        lore: 5,
        t: 40,
        wake: rgame::app::DayPhase::Night,
    };
    render(&atlas, &app);
    app.screen = Screen::World;
    app.player = app.zones[0].spawn;
    app.companion = app.zones[0].spawn;
    app.companion_prev = app.zones[0].spawn;

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
    for page in 0..rgame::content::wilds::GRIMOIRE_PAGES {
        app.screen = Screen::Grimoire { page };
        render(&atlas, &app);
    }
}
