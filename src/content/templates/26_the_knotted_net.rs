// ══════════════════════════════════════════════════════════════════
//   Quest 26: The Knotted Net                           ~ Mistholm ~
// ══════════════════════════════════════════════════════════════════
//
//   Netwright Halyard: "Every knot has its proper mending. Miss one
//   — just one — and the whole net unravels in a squall. I don't
//   miss knots. Neither will you."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   `match` takes a value and answers it arm by arm, one arm per
//   kind the value could be:
//
//       match knot {
//           Knot::Reef => String::from("..."),
//           Knot::Bowline => String::from("..."),
//           Knot::Granny => String::from("..."),
//       }
//
//   Each arm is `pattern => answer`. And here is the mercy of it:
//   forget an arm — any arm — and the compiler hands the net
//   straight back. A match must be *exhaustive*: every knot
//   answered, always. That's why matched nets don't unravel.
//
//   Fix `mending_for` with a match that answers all three knots:
//
//       Reef     ->  "tighten both loops"
//       Bowline  ->  "re-thread the rabbit hole"
//       Granny   ->  "cut it out and start over"
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

enum Knot {
    Reef,
    Bowline,
    Granny, // everyone ties one eventually; nobody admits it
}

fn mending_for(knot: Knot) -> String {
    // TODO: match every knot to its mending
    todo!()
}

fn main() {
    println!("A reef knot? {}", mending_for(Knot::Reef));
    println!("A bowline? {}", mending_for(Knot::Bowline));
    println!("A... granny knot? {}", mending_for(Knot::Granny));
}

// ─── Halyard's squall test (leave this part alone) ─────────────────
#[test]
fn every_knot_has_its_mending() {
    assert_eq!(mending_for(Knot::Reef), "tighten both loops");
    assert_eq!(mending_for(Knot::Bowline), "re-thread the rabbit hole");
    assert_eq!(mending_for(Knot::Granny), "cut it out and start over");
}
