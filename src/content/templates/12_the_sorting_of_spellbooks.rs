// ══════════════════════════════════════════════════════════════════
//   Quest 12: The Sorting of Spellbooks     ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   Sage Alderly: "Every spellbook belongs to a school, every
//   school has its shelf and its motto. A `match` handles it:
//   every school, every case, nothing forgotten — the rune itself
//   refuses to compile if you miss one."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Finish both match-runes. A match on an enum looks like:
//
//       match school {
//           School::Ember => 1,
//           School::Tide  => 2,
//           ...
//       }
//
//   Shelves:  Ember → 1, Tide → 2, Gale → 3, Stone → 4.
//   Mottos:   Ember → "warm hands, warm heart"
//             Tide  → "everything flows"
//             Gale  → "lightly now"
//             Stone → "patience, patience"
//
//   Replace both `todo!()`s, then press `c` in the game.
//   Your journey ends — or rather, begins — here.
// ──────────────────────────────────────────────────────────────────

enum School {
    Ember,
    Tide,
    Gale,
    Stone,
}

fn shelf_for(school: School) -> u32 {
    todo!("match every school to its shelf")
}

fn motto_for(school: School) -> &'static str {
    todo!("match every school to its motto")
}

fn main() {
    println!(
        "Shelf {} hums: '{}'",
        shelf_for(School::Ember),
        motto_for(School::Ember)
    );
}

// ─── The Library's final word (leave this part alone) ─────────────
#[test]
fn every_shelf_is_known() {
    assert_eq!(shelf_for(School::Ember), 1);
    assert_eq!(shelf_for(School::Tide), 2);
    assert_eq!(shelf_for(School::Gale), 3);
    assert_eq!(shelf_for(School::Stone), 4);
}

#[test]
fn every_motto_is_remembered() {
    assert_eq!(motto_for(School::Ember), "warm hands, warm heart");
    assert_eq!(motto_for(School::Tide), "everything flows");
    assert_eq!(motto_for(School::Gale), "lightly now");
    assert_eq!(motto_for(School::Stone), "patience, patience");
}
