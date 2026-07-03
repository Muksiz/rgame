// ══════════════════════════════════════════════════════════════════
//   Quest 8: Counting Fireflies                  ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Pip: "Wren says you helped light the big lantern! I've got a
//   little one — a jar — and I want it FULL before dark. One
//   firefly at a time, though. They won't be rushed."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A `while` loop keeps going for as long as its condition holds:
//
//       while some_condition {
//           // ...
//       }
//
//   Catch one firefly (`caught += 1`) on every trip through the
//   loop, and stop the moment the jar has `capacity` fireflies in
//   it — not before, not after.
//
//   Fix the rune, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn fill_the_jar(capacity: u32) -> u32 {
    let mut caught = 0;
    // TODO: a `while` loop that catches fireflies until the jar is full
    caught
}

fn main() {
    println!("Fireflies in the jar: {}", fill_the_jar(5));
}

// ─── Pip counts them twice, just to be sure (leave this part alone) ─
#[test]
fn the_jar_fills_up() {
    assert_eq!(fill_the_jar(5), 5);
}

#[test]
fn an_empty_jar_needs_no_catching() {
    assert_eq!(fill_the_jar(0), 0);
}
