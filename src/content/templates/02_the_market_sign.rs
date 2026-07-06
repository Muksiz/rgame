// ══════════════════════════════════════════════════════════════════
//   Quest 2: The Market Sign                     ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Tansy: "Grandmother's counting-rune tallies the apples one chalk
//   stroke at a time — one | per apple, bless it — but the sign out
//   front wants a proper NUMBER, and the rune point-blank refuses to
//   change kinds mid-thought."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   `apples` starts life as TEXT (the chalk tally) and must end up
//   a NUMBER (its stroke count). Plain assignment fails twice over:
//   the binding isn't `mut`, and even `mut` can't change a value's
//   type. The Rust way is *shadowing* — bind the SAME name again
//   with a fresh `let`, and it may carry a brand-new kind of value:
//
//       let apples = apples.len();
//
//   Fix the sign, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn market_sign() -> String {
    // Grandmother's counting-rune chalks one stroke per apple:
    let apples = "|||||||||||||||||||||";
    let slate = format!("tally: {apples}");

    // The sign out front wants the number, not the strokes.
    // One small word is missing at the start of this line:
    apples = apples.len();

    let pears = 7;
    format!("{slate} - {apples} apples and {pears} pears on the stall today")
}

fn main() {
    println!("{}", market_sign());
}

// ─── Tansy counts twice, just to be sure (leave this part alone) ──
#[test]
fn the_sign_reads_true() {
    assert_eq!(
        market_sign(),
        "tally: ||||||||||||||||||||| - 21 apples and 7 pears on the stall today"
    );
}
