// ══════════════════════════════════════════════════════════════════
//   Quest 29: The Tide-Pools                            ~ Mistholm ~
// ══════════════════════════════════════════════════════════════════
//
//   Grandmother Brine: "Most pools hold patience and nothing else,
//   and that's fine. But you can't hold a full formal match
//   ceremony over every puddle on the shore. Greet the one that
//   matters. Let the rest be."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   When only ONE shape matters, `if let` greets just that one:
//
//       if let Some(pearl) = pool {
//           // `pearl` is yours to name in here
//       } else {
//           // None? This side. No harm done.
//       }
//
//   And when a pearl is REQUIRED before work continues,
//   `let ... else` takes it in hand or steps out early:
//
//       let Some(pearl) = pool else {
//           return 0; // the else must leave the function
//       };
//       // from here on, `pearl` is simply in hand
//
//   1. `greet` — with if let (and its else):
//        Some(size)  ->  "oh, a pearl! 9 grains"  (its size)
//        None        ->  "just patience in this one"
//   2. `weigh` — with let-else: take the size in hand, or
//      return 0. Then hand the size back.
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn greet(pool: Option<u32>) -> String {
    todo!("if let Some(size), else the patient answer")
}

fn weigh(pool: Option<u32>) -> u32 {
    todo!("let Some(size) = pool else { return 0 };")
}

fn main() {
    println!("{}", greet(Some(9)));
    println!("{}", greet(None));
    println!("Total grains: {}", weigh(Some(9)) + weigh(None));
    println!("Grandmother Brine, at last, looks up.");
}

// ─── the grain scales (leave this part alone) ──────────────────────
#[test]
fn the_pearl_is_greeted() {
    assert_eq!(greet(Some(12)), "oh, a pearl! 12 grains");
    assert_eq!(greet(None), "just patience in this one");
}

#[test]
fn the_scales_are_honest() {
    assert_eq!(weigh(Some(31)), 31);
    assert_eq!(weigh(None), 0);
}
