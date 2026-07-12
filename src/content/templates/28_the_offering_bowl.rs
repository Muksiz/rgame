// ══════════════════════════════════════════════════════════════════
//   Quest 28: The Offering Bowl                         ~ Mistholm ~
// ══════════════════════════════════════════════════════════════════
//
//   Keeper Murre: "Some mornings the bowl holds a gift. Some
//   mornings it holds nothing. Both are answers — the trouble is
//   writing that down honestly."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Rust's word for maybe-a-thing is `Option<T>` — an enum from the
//   standard library, already imported in every scroll:
//
//       enum Option<T> {
//           Some(T),
//           None,
//       }
//
//   A bowl either holds `Some(gift)` or `None`, and the compiler
//   will not let you thank anyone until you've said WHICH. No
//   pretending the bowl is full; no forgetting that it might be.
//   `Some` and `None` are used bare — no `Option::` needed.
//
//   1. `shell_if_low_tide` — at low tide the sea leaves
//      Some(String::from("a pink shell")); otherwise None.
//   2. `thanks_for` — match both answers:
//        Some(gift)  ->  "the shrine thanks the sea for a pink shell"
//                        (whatever the gift was, woven in)
//        None        ->  "the shrine thanks the sea for the quiet"
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn shell_if_low_tide(low_tide: bool) -> Option<String> {
    todo!("Some shell at low tide, None otherwise")
}

fn thanks_for(offering: Option<String>) -> String {
    todo!("match: Some(gift) and None each get their words")
}

fn main() {
    println!("{}", thanks_for(shell_if_low_tide(true)));
    println!("{}", thanks_for(shell_if_low_tide(false)));
    println!("Both answers true. The shrine bell approves.");
}

// ─── the shrine's ledger (leave this part alone) ───────────────────
#[test]
fn the_sea_keeps_its_schedule() {
    assert_eq!(
        shell_if_low_tide(true),
        Some(String::from("a pink shell"))
    );
    assert_eq!(shell_if_low_tide(false), None);
}

#[test]
fn thanks_are_given_either_way() {
    assert_eq!(
        thanks_for(Some(String::from("a green pebble"))),
        "the shrine thanks the sea for a green pebble"
    );
    assert_eq!(
        thanks_for(None),
        "the shrine thanks the sea for the quiet"
    );
}
