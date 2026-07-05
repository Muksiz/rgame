// ══════════════════════════════════════════════════════════════════
//   Quest 6: The Deep, Deep Well               ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Well-keeper Bram: "Drop a pebble, count the heartbeats to the
//   splash. The old falling-stone rune knows the rest: five strides,
//   times the heartbeats, times the heartbeats again. But my rune
//   does the sum and then KEEPS THE ANSWER TO ITSELF."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   1. In a Rust function, the last line WITHOUT a semicolon is an
//      *expression* — its value is what the function returns. Add a
//      semicolon and it becomes a *statement*: the value is quietly
//      set down, and the function returns nothing. That stray
//      semicolon below is the whole problem.
//
//   2. Then settle the village argument: write a whole function
//      yourself, from nothing. `fn`, a name, a typed input, `->`
//      for what comes out:
//
//          fn deepest_in_the_valley(depth: u32) -> bool
//
//      Millbrook's well is eighteen strides. True when a depth
//      beats it.
//
//   Fix the rune, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn well_depth(seconds: u32) -> u32 {
    let strides = 5 * seconds * seconds;
    strides; // <- that semicolon sets the answer down instead of returning it
}

// TODO: write `deepest_in_the_valley` here — true past 18 strides.

fn main() {
    let depth = well_depth(2);
    println!("Two heartbeats down: {depth} strides!");
    println!("Deepest in the valley? {}", deepest_in_the_valley(depth));
}

// ─── The village's judgement (leave this part alone) ──────────────
#[test]
fn two_heartbeats() {
    assert_eq!(well_depth(2), 20, "5 * 2 * 2 strides");
}

#[test]
fn one_heartbeat() {
    assert_eq!(well_depth(1), 5);
}

#[test]
fn the_argument_is_settled() {
    assert!(deepest_in_the_valley(20), "twenty strides beats Millbrook's eighteen");
    assert!(!deepest_in_the_valley(18), "a tie is no victory");
    assert!(!deepest_in_the_valley(5));
}
