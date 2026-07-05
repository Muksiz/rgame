// ══════════════════════════════════════════════════════════════════
//   Quest 16: The Ferry Token               ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Ferryman Wick: "Every crossing gets a token — a number, a
//   holder, and a stamp once I've inspected it. My trouble is in
//   the ISSUING and the STAMPING."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   1. The issuing. When a variable's name already matches the
//      field's name, *field init shorthand* writes it once:
//
//          Token { number, holder, stamped: false }
//
//      (instead of `number: number, holder: holder` — Wick's eyes
//      cross at the long way.)
//
//   2. The stamping. Assigning to ANY field of an instance needs
//      the whole binding mutable: `let mut token = ...`. One little
//      word, and the stamp goes on.
//
//   Fix both, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

struct Token {
    number: u32,
    holder: String,
    stamped: bool,
}

fn issue(number: u32, holder: String) -> Token {
    // TODO: build the token — the parameter names already match the
    // field names, so the shorthand applies. Fresh tokens are unstamped.
    todo!()
}

fn crossing_paperwork() -> Token {
    // TODO: the stamp below changes the token — the binding must allow it.
    let token = issue(7, String::from("the rune-smith"));
    token.stamped = true;
    token
}

fn main() {
    let token = crossing_paperwork();
    println!(
        "Token No. {} — held by {}, {}.",
        token.number,
        token.holder,
        if token.stamped { "stamped and seaworthy" } else { "NOT VALID" }
    );
}

// ─── Riverside regulations (leave this part alone) ────────────────
#[test]
fn the_token_is_issued_correctly() {
    let token = issue(7, String::from("the rune-smith"));
    assert_eq!(token.number, 7);
    assert_eq!(token.holder, "the rune-smith");
    assert!(!token.stamped, "fresh tokens are unstamped until inspection");
}

#[test]
fn the_crossing_token_is_stamped() {
    let token = crossing_paperwork();
    assert_eq!(token.number, 7);
    assert!(token.stamped);
}
