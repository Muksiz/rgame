// ══════════════════════════════════════════════════════════════════
//   Quest 10: The Standard Baskets                ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Basket-weaver Briar: "Every basket comes past me for a
//   checking-stamp before it goes out. But my stamping-rune has
//   developed a HABIT: baskets go in, baskets do not come out."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Ownership flows through functions in both directions:
//
//   • Passing a `String` in MOVES it — the function owns it now.
//   • When the function ends, everything it still owns is dropped.
//   • Returning a value moves ownership back OUT to the caller.
//
//   Right now `stamp` takes the basket and returns nothing, so the
//   stamped basket is dropped — gone, mulch — the moment the
//   function ends. Give `stamp` a return type (`-> String`), hand
//   the stamped basket back on its last line, and catch it at the
//   call site.
//
//   Fix the round, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn stamp(basket: String) {
    // The stamped basket never leaves this function...
    let stamped = format!("{basket} [checked]");
}

fn morning_round() -> String {
    let basket = String::from("Basket no. 3");

    // TODO: `basket` moves into `stamp` here and is never handed back.
    stamp(basket);
    basket
}

fn main() {
    println!("Back from the stamping table: '{}'", morning_round());
}

// ─── Briar counts her baskets by hand (leave this part alone) ─────
#[test]
fn the_basket_comes_back_stamped() {
    assert_eq!(morning_round(), "Basket no. 3 [checked]");
}
