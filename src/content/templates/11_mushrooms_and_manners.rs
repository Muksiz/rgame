// ══════════════════════════════════════════════════════════════════
//   Quest 11: Mushrooms & Manners               ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Forager Maren's rules, in order of importance:
//     1. If it GLOWS — never. No exceptions. I don't care how pretty.
//     2. More than four spots — leave it.
//     3. Otherwise — straight into the basket.
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Turn Maren's rules into code with `if` and `else`. In Rust,
//   `if` is an expression — it can *be* the answer:
//
//       if something { true } else { false }
//
//   (Seasoned rune-smiths may notice the whole thing can be one
//   boolean expression. Maren accepts both styles.)
//
//   Replace the `todo!(...)`, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn is_safe_to_eat(spots: u32, glows: bool) -> bool {
    todo!("apply Maren's rules, glow-check first")
}

fn main() {
    println!(
        "Plain and three-spotted? {}",
        if is_safe_to_eat(3, false) { "basket!" } else { "leave it." }
    );
}

// ─── Maren's basket-check (leave this part alone) ─────────────────
#[test]
fn plain_mushrooms_are_dinner() {
    assert!(is_safe_to_eat(0, false));
    assert!(is_safe_to_eat(4, false));
}

#[test]
fn spotty_mushrooms_stay_put() {
    assert!(!is_safe_to_eat(5, false));
    assert!(!is_safe_to_eat(11, false));
}

#[test]
fn glowing_mushrooms_are_never_ever_dinner() {
    assert!(!is_safe_to_eat(0, true));
    assert!(!is_safe_to_eat(9, true));
}
