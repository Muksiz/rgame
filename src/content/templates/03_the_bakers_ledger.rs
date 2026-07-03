// ══════════════════════════════════════════════════════════════════
//   Quest 3: The Baker's Ledger                ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Baker Poppy: "Twelve loaves this morning, nine more tonight,
//   four set aside for the festival. But the ledger-rune insists
//   the number cannot change once written!"
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   In Rust, variables are *immutable* by default: once bound,
//   `loaves` may never change... unless you ask politely with
//   the keyword `mut`.
//
//   Make the ledger add up, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn loaves_by_evening() -> i32 {
    let loaves = 12; // the morning batch

    // The evening batch goes in...
    loaves = loaves + 9;

    // ...and four are set aside for the festival table.
    loaves - 4
}

fn main() {
    println!("The ledger reads: {} loaves.", loaves_by_evening());
}

// ─── Poppy's arithmetic (leave this part alone) ───────────────────
#[test]
fn the_ledger_adds_up() {
    assert_eq!(loaves_by_evening(), 17, "12 + 9 - 4 loaves should be 17");
}
