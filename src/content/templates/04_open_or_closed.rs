// ══════════════════════════════════════════════════════════════════
//   Quest 4: Open or Closed                       ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Watchman Fitch: "One rule, cleanly written, that's all I ask.
//   The toll is waived for children under twelve, and for anyone
//   who lives here — locals don't pay to enter their own village.
//   Whichever reason fits, waive it. Both, neither, I don't mind."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Rust's `bool` is `true` or `false`, and you build one straight
//   out of comparisons (`<`, `>=`, `==`, ...) joined with `&&` (and)
//   or `||` (or). No `if` needed here — the comparison itself IS
//   the answer.
//
//   Fix the rune, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn toll_is_waived(age: u32, is_local: bool) -> bool {
    // TODO: true if age is under 12, OR if is_local is true
    false
}

fn main() {
    println!("Waived for an 8-year-old? {}", toll_is_waived(8, false));
}

// ─── Fitch checks every gate-crosser (leave this part alone) ──────
#[test]
fn children_pass_free() {
    assert!(toll_is_waived(8, false));
}

#[test]
fn locals_pass_free() {
    assert!(toll_is_waived(30, true));
}

#[test]
fn everyone_else_pays() {
    assert!(!toll_is_waived(30, false));
}
