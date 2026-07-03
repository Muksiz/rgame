// ══════════════════════════════════════════════════════════════════
//   Quest 10: The Standard Baskets                ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Basket-weaver Briar: "Every stall gets four baskets, always in
//   the same order, always the same four sizes. I just need to know
//   what the THIRD one holds. Not the fourth. The third."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A fixed-size array like `[5, 6, 7, 8]` has type `[i32; 4]` — four
//   slots, no more, no less — and counting starts at zero:
//
//       basket[0]  // the first basket
//       basket[1]  // the second
//       ...
//
//   Fix the index so it reaches the THIRD basket, then press `c`.
// ──────────────────────────────────────────────────────────────────

fn basket_capacities() -> [u32; 4] {
    [5, 6, 7, 8]
}

fn third_basket_capacity() -> u32 {
    let baskets = basket_capacities();
    baskets[3] // TODO: array indices start at 0 — this reaches the fourth basket
}

fn main() {
    println!("The third basket holds {}.", third_basket_capacity());
}

// ─── Briar counts her baskets by hand (leave this part alone) ─────
#[test]
fn there_are_four_standard_baskets() {
    assert_eq!(basket_capacities(), [5, 6, 7, 8]);
}

#[test]
fn the_third_basket_is_the_third() {
    assert_eq!(third_basket_capacity(), 7);
}
