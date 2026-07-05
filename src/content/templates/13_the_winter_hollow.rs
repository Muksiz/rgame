// ══════════════════════════════════════════════════════════════════
//   Quest 13: The Winter Hollow                  ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Hollow-keeper Yew: "The labeling-rune writes the hoard's label
//   INSIDE itself, then tries to hand me a mere borrow of it. But
//   everything inside the rune is swept away the moment it
//   finishes! A finger pointing at swept-away writing!"
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A function can never return a reference to something it created
//   inside itself: the value is dropped when the function ends, and
//   the reference would point at nothing. The compiler refuses to
//   let such a thing exist — read its note below, it's a good one.
//
//   The honest fix is to hand out the value itself:
//
//       fn hoard_label(acorns: u32) -> String   // owned, not borrowed
//
//   ...and return `label`, not `&label`. Ownership then MOVES out
//   of the function and into the caller's hands. Nothing is copied;
//   nothing dangles.
//
//   Mend the rune, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn hoard_label(acorns: u32) -> &String {
    let label = format!("{acorns} acorns against the winter");
    // TODO: `label` is swept away when this function ends — hand out
    // the owned String itself, not a borrow of it.
    &label
}

fn main() {
    println!("The hoard-post reads: '{}'", hoard_label(31));
}

// ─── Yew checks the hollow every evening (leave this part alone) ──
#[test]
fn the_label_survives_the_rune() {
    assert_eq!(hoard_label(31), "31 acorns against the winter");
}

#[test]
fn even_a_lean_year_gets_its_label() {
    assert_eq!(hoard_label(3), "3 acorns against the winter");
}
