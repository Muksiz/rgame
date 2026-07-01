//! The solve-through test: proves every quest is honest.
//!
//! For each of the 12 quests it runs the *real* checker (rustc --test) twice:
//! the untouched template must NOT pass, and the reference solution in
//! `tests/solutions/` must pass. Compiles 24 files, so it takes a little while.

use std::path::Path;

use rgame::checker::{Outcome, check_file};
use rgame::content::quests::QUESTS;

#[test]
fn every_template_fails_and_every_solution_passes() {
    let dir = std::env::temp_dir().join("rune_road_solve_through");
    std::fs::create_dir_all(&dir).unwrap();

    for quest in &QUESTS {
        // 1. The scaffolded template must not already be a passing quest.
        let template_path = dir.join(format!("template_{}", quest.file_name));
        std::fs::write(&template_path, quest.template).unwrap();
        let outcome = check_file(&template_path, &format!("template_{:02}", quest.id));
        assert!(
            !matches!(outcome, Outcome::Pass { .. }),
            "quest {} ({}) passes without the player doing anything!",
            quest.id,
            quest.file_name
        );

        // 2. The reference solution must pass.
        let solution_src = Path::new("tests/solutions").join(quest.file_name);
        let solution = std::fs::read_to_string(&solution_src).unwrap_or_else(|e| {
            panic!("missing reference solution {}: {e}", solution_src.display())
        });
        let solution_path = dir.join(format!("solution_{}", quest.file_name));
        std::fs::write(&solution_path, solution).unwrap();
        let outcome = check_file(&solution_path, &format!("solution_{:02}", quest.id));
        assert!(
            matches!(outcome, Outcome::Pass { .. }),
            "reference solution for quest {} ({}) does not pass: {:?}",
            quest.id,
            quest.file_name,
            outcome
        );
    }
}
