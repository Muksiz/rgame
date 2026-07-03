// ══════════════════════════════════════════════════════════════════
//   Quest 2: The Market Sign                     ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Tansy: "Nine more apples came in from the orchard cart, but my
//   sign-rune won't say so. And it's completely forgotten the pears!
//   One thing at a time, sign-rune. ONE THING AT A TIME."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Two small things, both on the same sign:
//
//   1. `apples` needs the new nine counted in. You can bind the SAME
//      name again with a new `let` — this is called *shadowing*, and
//      it quietly replaces the old value with a new one:
//
//          let apples = apples + 9;
//
//   2. The sign only ever mentions apples. `format!`/`println!` can
//      take more than one value — just add another `{}` (or, tidier,
//      name the value right inside the braces: `{pears}`).
//
//   Fix the sign, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn market_tally() -> String {
    let apples = 12;
    // Nine more apples arrived from the orchard cart — same name, new count.
    let pears = 7;

    format!("{} apples on the stall today", apples)
}

fn main() {
    println!("{}", market_tally());
}

// ─── Tansy counts twice, just to be sure (leave this part alone) ──
#[test]
fn the_sign_reads_true() {
    assert_eq!(market_tally(), "21 apples and 7 pears on the stall today");
}
