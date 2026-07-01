// ══════════════════════════════════════════════════════════════════
//   Quest 9: A Message in a Bottle         ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Hermit Morrow: "The river brought me a letter. In two pieces,
//   as usual. Word-slices and owned words are different creatures:
//   `&str` is a window onto words; `String` is words you keep.
//   The mending must produce the keeping kind."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Join the two borrowed halves into one owned `String`, with a
//   single space between them. The easiest mending-tool:
//
//       format!("{first} {second}")
//
//   Replace the `todo!()`, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn mend_message(first_half: &str, second_half: &str) -> String {
    todo!()
}

fn sign_message(message: String) -> String {
    format!("{message} — with love, M.")
}

fn the_whole_letter() -> String {
    let mended = mend_message("meet me", "where the river sings");
    sign_message(mended)
}

fn main() {
    println!("{}", the_whole_letter());
}

// ─── Morrow's careful reading (leave this part alone) ─────────────
#[test]
fn the_halves_are_mended() {
    assert_eq!(
        mend_message("meet me", "where the river sings"),
        "meet me where the river sings"
    );
}

#[test]
fn the_letter_is_signed() {
    assert!(the_whole_letter().ends_with("— with love, M."));
}
