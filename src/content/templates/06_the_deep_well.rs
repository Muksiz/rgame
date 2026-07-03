// ══════════════════════════════════════════════════════════════════
//   Quest 6: The Deep, Deep Well               ~ Emberwick Village ~
// ══════════════════════════════════════════════════════════════════
//
//   Well-keeper Bram: "Drop a pebble, count the heartbeats to the
//   splash. The old falling-stone rune knows the rest: half of
//   9.8, times the seconds, times the seconds again."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Rust never mixes number types silently: an `i32` (whole
//   number) refuses to multiply with an `f64` (decimal number)
//   until you *convert* one of them — with `as`.
//
//   For example:  `seconds as f64`
//
//   Make the depths compute, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn well_depth(seconds: i32) -> f64 {
    // depth = ½ · 9.8 · t · t   ...but the types don't agree yet:
    0.5 * 9.8 * seconds * seconds
}

fn main() {
    println!("Two heartbeats down: {} strides deep!", well_depth(2));
}

// ─── The village's judgement (leave this part alone) ──────────────
#[test]
fn two_heartbeats() {
    assert!((well_depth(2) - 19.6).abs() < 0.01, "two heartbeats ≈ 19.6");
}

#[test]
fn one_heartbeat() {
    assert!((well_depth(1) - 4.9).abs() < 0.01, "one heartbeat ≈ 4.9");
}
