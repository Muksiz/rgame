// ══════════════════════════════════════════════════════════════════
//   Quest 14: The Lost Bell                       ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Woodward Sable: "Every ground I search for the bell gets one
//   letter on my planning-strip — R for the ridge, H for the
//   hollow, F for the ford, on down the week. An evening's walking
//   covers exactly THREE grounds. My rune plans the whole strip."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Two repairs, both about borrowing text:
//
//   1. A *slice* borrows part of a String without copying it:
//      `&strip[..3]` is a window onto the first three letters.
//      (`&strip[..]` would be a window onto the whole thing —
//      that's the rune's current appetite.)
//
//   2. The parameter type `&String` only accepts borrowed Strings.
//      Make it `&str` instead and it welcomes BOTH kinds of text —
//      a borrowed String or a plain written scrap. (Sable's tests
//      hand it both.)
//
//   Mend the rune, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn tonights_grounds(strip: &String) -> &str {
    // TODO: one evening is THREE grounds, not the whole strip
    &strip[..]
}

fn plan_the_search() -> String {
    // Ridge, Hollow, Ford, Glade, Pond, Waterfall — a week of grounds.
    let strip = String::from("RHFGPW");
    let tonight = tonights_grounds(&strip);
    format!("tonight: {tonight}")
}

fn main() {
    println!("{}", plan_the_search());
}

// ─── Sable makes the rounds every dusk (leave this part alone) ────
#[test]
fn an_evening_is_three_grounds() {
    assert_eq!(plan_the_search(), "tonight: RHF");
}

#[test]
fn a_scrap_of_writing_works_too() {
    // A string literal is already a &str — a welcoming rune takes it as-is.
    assert_eq!(tonights_grounds("GPWRHF"), "GPW");
}
