// ══════════════════════════════════════════════════════════════════
//   Quest 4: Open or Closed                       ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Watchman Fitch: "Under twelve or a local — free through. Seventy
//   or older — half toll, two coins. Everyone else pays the full
//   four. Simple for a human. Apparently baffling for a rune."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Two small repairs:
//
//   1. Rust's `bool` is `true` or `false`, and comparisons like
//      `age < 12` already produce one. Join two conditions with
//      `||` (or) / `&&` (and) — no `if` needed, the comparison
//      itself IS the answer.
//
//   2. `if` is an *expression* — it produces a value, so it can sit
//      right inside a `let`. Chain a middle case with `else if`.
//
//   Fix both, then return to the game and press `c` to cast.
// ──────────────────────────────────────────────────────────────────

fn toll_is_waived(age: u32, is_local: bool) -> bool {
    // TODO: true if age is under 12, OR if is_local is true
    false
}

fn toll_owed(age: u32, is_local: bool) -> u32 {
    // The whole `if` is one expression; its value lands in `coins`.
    let coins = if toll_is_waived(age, is_local) {
        0
    } else {
        // TODO: elders (70 or older) pay half — 2 coins. Everyone
        // else pays the full 4. You'll want an `else if` in here.
        4
    };
    coins
}

fn main() {
    println!("An 8-year-old owes {} coins.", toll_owed(8, false));
    println!("A 30-year-old trader owes {} coins.", toll_owed(30, false));
}

// ─── Fitch checks every gate-crosser (leave this part alone) ──────
#[test]
fn children_pass_free() {
    assert!(toll_is_waived(8, false));
    assert_eq!(toll_owed(8, false), 0);
}

#[test]
fn locals_pass_free() {
    assert!(toll_is_waived(30, true));
    assert_eq!(toll_owed(75, true), 0, "a local elder is waived, not halved");
}

#[test]
fn elders_pay_half() {
    assert_eq!(toll_owed(70, false), 2);
    assert_eq!(toll_owed(91, false), 2);
}

#[test]
fn everyone_else_pays_in_full() {
    assert!(!toll_is_waived(30, false));
    assert_eq!(toll_owed(30, false), 4);
}
