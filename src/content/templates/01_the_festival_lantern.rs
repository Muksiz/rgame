// ══════════════════════════════════════════════════════════════════
//   Quest 1: The Festival Lantern              ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Elder Rowan: "The great lantern only lights when the old words
//   are spoken *exactly* right. I always forget the little mark at
//   the end. Would you try, dear?"
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   The spell below is missing something small but important.
//   In Rust, `println!` is a *macro* — and macros are always
//   called with a `!` after their name.
//
//   Fix the spell so it compiles, then return to the game
//   and press `c` to cast it.
// ──────────────────────────────────────────────────────────────────

fn main() {
    // Fix this line:
    println("Let there be light!");
}

// ─── The lantern's judgement (leave this part alone) ──────────────
#[test]
fn the_lantern_listens() {
    // If the old words compile, the lantern will hear them.
    main();
}
