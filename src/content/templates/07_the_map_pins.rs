// ══════════════════════════════════════════════════════════════════
//   Quest 7: The Map Pins                         ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Cartographer Reed: "A landmark is two numbers traveling
//   together. The road is four legs in a fixed row. And my survey
//   chain is five paces, thrown end over end until the distance is
//   covered. Get all three right and the map can be TRUSTED."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   The last of Emberwick's troubles, in four small pieces:
//
//   • A tuple bundles a fixed handful of values: `(12, 4)`. Take
//     one apart by destructuring — `let (col, row) = pin;` — or by
//     position: `pin.0`, `pin.1`.
//
//   • An array is a fixed row of one type: `[i32; 4]` holds exactly
//     four. A `for` loop visits each element in order:
//
//         for leg in legs { ... }        // or: for i in 0..4 { ... }
//
//   • A `while` loop runs as long as its condition holds:
//
//         while covered < distance { ... }
//
//   Fill in the four runes, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn well_pin() -> (i32, i32) {
    // TODO: the well sits at column 12, row 4 — build that pin
    (0, 0)
}

fn pin_code(pin: (i32, i32)) -> i32 {
    // Reed files every pin as a code: column times a hundred, plus row.
    // TODO: destructure `pin` and build the code — (12, 4) files as 1204
    todo!()
}

fn road_length(legs: [i32; 4]) -> i32 {
    // TODO: walk the array with a `for` loop and add the legs up
    todo!()
}

fn chain_throws(distance: i32) -> i32 {
    // TODO: the chain covers 5 paces a throw — `while` the distance
    // isn't covered yet, throw again and count it. How many throws?
    todo!()
}

fn main() {
    println!("The well is filed under {}.", pin_code(well_pin()));
    println!("The east road runs {} paces.", road_length([3, 7, 2, 5]));
    println!("Survey says: {} throws of the chain.", chain_throws(23));
}

// ─── Reed checks every pin against the map (leave this part alone) ─
#[test]
fn the_well_is_pinned() {
    assert_eq!(well_pin(), (12, 4));
}

#[test]
fn the_pin_files_correctly() {
    assert_eq!(pin_code((12, 4)), 1204);
    assert_eq!(pin_code((3, 9)), 309);
}

#[test]
fn the_road_sums_true() {
    assert_eq!(road_length([3, 7, 2, 5]), 17);
    assert_eq!(road_length([1, 1, 1, 1]), 4);
}

#[test]
fn the_chain_count_lands_exact() {
    assert_eq!(chain_throws(23), 5, "four throws cover 20; a fifth finishes 23");
    assert_eq!(chain_throws(20), 4, "20 paces is exactly four throws");
    assert_eq!(chain_throws(0), 0, "no distance, no throwing");
}
