//! The full-journey test: plays the entire game start to finish through the
//! same input path a player uses — walk to each NPC, talk, receive the
//! scaffolded quest file, "solve" it by writing the reference solution, cast,
//! celebrate, cross each gate — all the way to the epilogue.
//!
//! Runs in a temp working directory so the real quests/ and save.json are
//! never touched.

use std::time::{Duration, Instant};

use rgame::app::{App, Key, Screen};
use rgame::checker::QUEST_DIR;
use rgame::content::quests::QUESTS;

fn key(app: &mut App, code: Key) {
    app.on_key(code);
}

/// Keep pressing Enter until the dialogue closes (reveal, page, close).
fn click_through_dialogue(app: &mut App) {
    for _ in 0..40 {
        if !matches!(app.screen, Screen::Dialogue(_)) {
            return;
        }
        key(app, Key::Enter);
    }
    panic!("dialogue never ended");
}

/// Park the player on a walkable tile next to `pos`.
fn stand_next_to(app: &mut App, pos: (i32, i32)) {
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1)] {
        let spot = (pos.0 + dx, pos.1 + dy);
        if app.zone().tile(spot.0, spot.1).walkable() && app.zone().npc_at(spot.0, spot.1).is_none()
        {
            app.player = spot;
            return;
        }
    }
    panic!("nowhere to stand next to {pos:?}");
}

fn wait_for_cast_result(app: &mut App) {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        app.on_tick();
        match app.screen {
            Screen::CastResult { .. } => return,
            _ if Instant::now() > deadline => panic!("cast never finished"),
            _ => std::thread::sleep(Duration::from_millis(10)),
        }
    }
}

#[test]
fn the_whole_journey_can_be_walked() {
    let solutions = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/solutions");
    let playdir = std::env::temp_dir().join("rune_road_journey");
    let _ = std::fs::remove_dir_all(&playdir);
    std::fs::create_dir_all(&playdir).unwrap();
    std::env::set_current_dir(&playdir).unwrap();

    let mut app = App::new();
    app.screen = Screen::Title { selected: 0 };
    key(&mut app, Key::Enter); // A new journey -> the character chooser
    assert!(matches!(app.screen, Screen::CharSelect { .. }));
    key(&mut app, Key::Right); // pick a different look, just to exercise it
    for c in ['j', 'a', 'n', 'i'] {
        key(&mut app, Key::Char(c)); // a blank name won't set off, so name yourself
    }
    key(&mut app, Key::Enter); // accept the name and set off
    assert!(matches!(app.screen, Screen::World));
    assert!(!app.player_name.is_empty(), "the traveller should be named");

    for (i, quest) in QUESTS.iter().enumerate() {
        // Walk (well, arrive) at the quest giver and hear them out.
        assert_eq!(
            app.zone_idx, quest.zone,
            "wrong zone for quest {}",
            quest.id
        );
        let npc_pos = app
            .zone()
            .npcs
            .iter()
            .find(|n| n.quest == Some(quest.id))
            .unwrap()
            .pos;
        stand_next_to(&mut app, npc_pos);
        key(&mut app, Key::Char('e'));
        assert!(
            matches!(app.screen, Screen::Dialogue(_)),
            "no dialogue from {} for quest {}",
            quest.npc,
            quest.id
        );
        click_through_dialogue(&mut app);
        let scroll = playdir.join(QUEST_DIR).join(quest.file_name);
        assert!(scroll.exists(), "quest {} was not scaffolded", quest.id);

        // A fresh cast must fizzle (the template is not a solution).
        key(&mut app, Key::Char('c'));
        wait_for_cast_result(&mut app);
        let Screen::CastResult { outcome, .. } = &app.screen else {
            unreachable!()
        };
        assert!(
            !matches!(outcome, rgame::checker::Outcome::Pass { .. }),
            "quest {} passed unedited",
            quest.id
        );
        key(&mut app, Key::Enter);

        // "Solve" it in the editor.
        std::fs::copy(solutions.join(quest.file_name), &scroll).unwrap();
        key(&mut app, Key::Char('c'));
        wait_for_cast_result(&mut app);
        let Screen::CastResult { outcome, .. } = &app.screen else {
            unreachable!()
        };
        assert!(
            matches!(outcome, rgame::checker::Outcome::Pass { .. }),
            "quest {} solution rejected: {outcome:?}",
            quest.id
        );
        key(&mut app, Key::Enter); // into the success dialogue
        click_through_dialogue(&mut app);
        assert!(app.completed.contains(&quest.id));

        // End of a zone: cross the gate east (or arrive at the epilogue).
        let is_finale = i + 1 == QUESTS.len();
        if is_finale {
            assert!(
                matches!(app.screen, Screen::Epilogue { .. }),
                "finishing quest {} should roll the epilogue",
                quest.id
            );
            for _ in 0..10 {
                key(&mut app, Key::Enter);
            }
            assert!(matches!(app.screen, Screen::World));
        } else if QUESTS[i + 1].zone != quest.zone {
            let gate = app.zone().gate.unwrap();
            let before = app.zone_idx;
            app.player = (gate.0 - 1, gate.1);
            key(&mut app, Key::Right);
            assert_eq!(
                app.zone_idx,
                before + 1,
                "gate after quest {} stayed shut",
                quest.id
            );
        }
    }

    assert!(
        playdir.join("save.json").exists(),
        "the journey was never saved"
    );
    assert_eq!(app.completed.len(), QUESTS.len());
}
