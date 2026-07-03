// ══════════════════════════════════════════════════════════════════
//   Quest 9: A Spell for Wren                  ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Wren: "A spell that makes every step count DOUBLE! Twice the
//   walking with the same feet! Grandmother says a function is a
//   little machine with a door in and a door out."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Write the whole function yourself this time. The shape of a
//   Rust function is:
//
//       fn name(input: Type) -> ReturnType {
//           // the last expression (no semicolon) is returned
//       }
//
//   Wren's spell is called `double_step`. It takes the paces as
//   an `i32` and returns them doubled, also as an `i32`.
//
//   Build it below, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

// TODO: write `double_step` here.

fn main() {
    println!("Seven paces become {}!", double_step(7));
}

// ─── Wren's hop-test (leave this part alone) ──────────────────────
#[test]
fn small_hops() {
    assert_eq!(double_step(3), 6);
}

#[test]
fn a_grand_journey() {
    assert_eq!(double_step(21), 42);
}
