// ══════════════════════════════════════════════════════════════════
//   Quest 8: Counting Fireflies                  ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Pip: "Every firefly I catch gets a tally-mark scratched onto the
//   jar's label. That's the RULES. But the label-rune won't take the
//   marks — the label it starts with can't grow!"
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Rust has two kinds of text. A string literal like "fireflies: "
//   is carved once, borrowed, and can NEVER grow. A `String` is the
//   *keeping* kind: it owns its letters on the heap and grows as
//   needed.
//
//       String::from("fireflies: ")   // literal -> String
//       label.push_str("*")           // append (needs `mut`)
//
//   The label below starts as a literal, so the tallies won't stick.
//   Make it a String, then press `c` in the game.
//
//   (When `label`'s owner goes out of scope, Rust frees it all by
//   itself — that tidying-up is called `drop`, and it's nobody's
//   chore. Pip approves of magic that cleans up after itself.)
// ──────────────────────────────────────────────────────────────────

fn jar_label(caught: u32) -> String {
    // TODO: this label is the never-growing kind. String::from(...) it.
    let mut label = "fireflies: ";
    for _ in 0..caught {
        label.push_str("*");
    }
    label
}

fn main() {
    println!("The jar reads: '{}'", jar_label(3));
}

// ─── Pip counts them twice, just to be sure (leave this part alone) ─
#[test]
fn every_catch_leaves_a_mark() {
    assert_eq!(jar_label(3), "fireflies: ***");
    assert_eq!(jar_label(5), "fireflies: *****");
}

#[test]
fn an_empty_jar_is_still_labeled() {
    assert_eq!(jar_label(0), "fireflies: ");
}
