// ══════════════════════════════════════════════════════════════════
//   Quest 11: Mushrooms & Manners               ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Forager Maren's rules, in order of importance:
//     1. If it GLOWS — never. No exceptions. I don't care how pretty.
//     2. More than four spots — leave it.
//     3. Otherwise — straight into the basket.
//
//   "The rune applies the rules fine. The trouble is it TAKES each
//   mushroom to inspect it, and inspected mushrooms never come
//   back. My dinner! In rune-land!"
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   The rune only needs to LOOK. A *shared borrow* — `&String` —
//   lets a function read a value it never owns, so the caller keeps
//   it. Two small edits:
//
//       fn is_dinner(mushroom: &String, spots: u32) -> bool    // borrow...
//       is_dinner(&find, 3)                                    // ...and lend
//
//   Fix both ends, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn is_dinner(mushroom: String, spots: u32) -> bool {
    // Maren's rules: no glow (checked FIRST), at most four spots.
    !mushroom.contains("glowing") && spots <= 4
}

fn basket_check() -> String {
    let find = String::from("a plain brown bolete");

    // TODO: this call MOVES `find` into the rune — lend it instead.
    let verdict = is_dinner(find, 3);

    if verdict {
        format!("{find} — into the basket")
    } else {
        format!("{find} — left for the moths")
    }
}

fn main() {
    println!("{}", basket_check());
}

// ─── Maren's basket-check (leave this part alone) ─────────────────
#[test]
fn the_mushroom_survives_inspection() {
    assert_eq!(basket_check(), "a plain brown bolete — into the basket");
}

#[test]
fn plain_mushrooms_are_dinner() {
    assert!(is_dinner(&String::from("a plain brown bolete"), 0));
    assert!(is_dinner(&String::from("a stout penny-cap"), 4));
}

#[test]
fn spotty_mushrooms_stay_put() {
    assert!(!is_dinner(&String::from("a freckled toadstool"), 5));
    assert!(!is_dinner(&String::from("a freckled toadstool"), 11));
}

#[test]
fn glowing_mushrooms_are_never_ever_dinner() {
    assert!(!is_dinner(&String::from("a glowing violet cap"), 0));
    assert!(!is_dinner(&String::from("a glowing violet cap"), 9));
}
