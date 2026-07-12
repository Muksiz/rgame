// ══════════════════════════════════════════════════════════════════
//   Quest 24: The Tide Board                            ~ Mistholm ~
// ══════════════════════════════════════════════════════════════════
//
//   Tidewatcher Nerine: "The board says the tide is coming in AND
//   going out. Boards shouldn't be allowed to say that. A tide is
//   ONE of two things — I need it carved so the board can't lie."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   When a value is exactly one of a fixed set of kinds, Rust says
//   so with an *enum*:
//
//       enum Tide {
//           Ebb,
//           Flood,
//       }
//
//   The kinds are its *variants*, and a value is written with `::`
//   — `Tide::Ebb`. An enum value IS one variant. It can never be
//   two at once; the compiler enforces the sea's own manners.
//
//   1. Give the Tide enum its two variants: `Ebb` and `Flood`.
//   2. Fix `turn` so each tide becomes the other — Ebb turns to
//      Flood, Flood turns to Ebb. Plain if/else works fine (the
//      derive line up top is what makes tides comparable with ==).
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tide {
    // TODO: the two kinds a tide can be
}

/// The tide that follows this one. The sea has never once varied.
fn turn(tide: Tide) -> Tide {
    todo!("Ebb turns to Flood, Flood turns to Ebb")
}

fn main() {
    let morning = Tide::Ebb;
    let midday = turn(morning);
    println!("Morning: {morning:?}. Midday: {midday:?}.");
    println!("Evening: {:?}. The board can't lie anymore.", turn(midday));
}

// ─── Nerine's tide tables (leave this part alone) ──────────────────
#[test]
fn the_tide_turns() {
    assert_eq!(turn(Tide::Ebb), Tide::Flood);
    assert_eq!(turn(Tide::Flood), Tide::Ebb);
}

#[test]
fn two_turns_bring_it_home() {
    assert_eq!(turn(turn(Tide::Ebb)), Tide::Ebb);
}
