// ══════════════════════════════════════════════════════════════════
//   Quest 7: The Ferry Token               ~ Silverford Riverlands ~
// ══════════════════════════════════════════════════════════════════
//
//   Ferryman Wick: "I inspect your token, I write you a receipt,
//   and you get the token BACK. But my inspection-rune is greedy —
//   hand it the token and the token is just... gone. 'Moved', it
//   says, like that's an apology."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   In Rust every value has exactly one owner. Passing a `String`
//   into a function *moves* it — the caller can't use it again.
//
//   Teach the rune to LOOK without KEEPING: make `show_token`
//   borrow (`&str`) instead of take (`String`), and lend the token
//   with `&` at the call site.
//
//   Fix it, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn show_token(token: String) -> String {
    format!("Wick squints at it: '{token}'. Genuine!")
}

fn crossing_paperwork() -> (String, String) {
    let token = String::from("Silverford Token No. 7");

    // The inspection swallows the token whole...
    let receipt = show_token(token);

    // ...so it can't be given back here. Poor form, rune. Poor form.
    (token, receipt)
}

fn main() {
    let (token, receipt) = crossing_paperwork();
    println!("{receipt}");
    println!("Returned to your pocket: {token}");
}

// ─── Riverside regulations (leave this part alone) ────────────────
#[test]
fn the_token_comes_back() {
    let (token, _) = crossing_paperwork();
    assert_eq!(token, "Silverford Token No. 7");
}

#[test]
fn the_receipt_is_written() {
    let (_, receipt) = crossing_paperwork();
    assert!(receipt.contains("Genuine"));
}
