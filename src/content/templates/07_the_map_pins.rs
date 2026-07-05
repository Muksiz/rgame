// ══════════════════════════════════════════════════════════════════
//   Quest 7: The Map Pins                         ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Cartographer Reed: "A landmark is two numbers traveling together.
//   The road east is four legs in a fixed row. Pin the one and sum the
//   other, and the map can be TRUSTED."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Two small runes, and the map is honest:
//
//   • A tuple bundles a fixed handful of values: `(12, 4)`. Read one
//     back by position — `pin.0`, `pin.1` — or by destructuring,
//     `let (col, row) = pin;`.
//
//   • An array is a fixed row of one type: `[i32; 4]` holds exactly
//     four. A `for` loop visits each element in order to add them up:
//
//         for leg in legs { total += leg; }
//
//   Fill in the two runes, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn well_pin() -> (i32, i32) {
    // TODO: the well sits at column 12, row 4 — build that pin as a tuple
    (0, 0)
}

fn road_length(legs: [i32; 4]) -> i32 {
    // TODO: walk the array with a `for` loop and add the four legs up
    todo!()
}

fn main() {
    let pin = well_pin();
    // A tuple is taken apart by position — `pin.0` is the column, `pin.1` the row.
    println!("The well is pinned at column {}, row {}.", pin.0, pin.1);
    println!("The east road runs {} paces.", road_length([3, 7, 2, 5]));
}

// ─── Reed checks every pin against the map (leave this part alone) ─
#[test]
fn the_well_is_pinned() {
    assert_eq!(well_pin(), (12, 4));
}

#[test]
fn the_road_sums_true() {
    assert_eq!(road_length([3, 7, 2, 5]), 17);
    assert_eq!(road_length([1, 1, 1, 1]), 4);
}
