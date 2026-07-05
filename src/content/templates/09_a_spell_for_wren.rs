// ══════════════════════════════════════════════════════════════════
//   Quest 9: A Spell for Wren                  ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Wren: "I wrote my FIRST incantation — 'double step'! — and I
//   want to cast it TWICE, once for each foot, obviously. But the
//   casting-rune EATS it on the first go! Grandmother says that's
//   ownership. I say it's rude."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Grandmother is right: a `String` has exactly one owner, and
//   passing it into a function *moves* it there for good — after
//   the first `cast(spell)`, the spell is gone from this scope.
//
//   The honest fix: `.clone()` makes an independent copy, letters
//   and all. Feed the FIRST cast a clone, and the original survives
//   for the second:
//
//       let first = cast(spell.clone());
//
//   Fix the practice session, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn cast(incantation: String) -> String {
    format!("~ {incantation} ~")
}

fn practice_twice() -> (String, String) {
    let spell = String::from("double step");

    // TODO: this first cast eats the spell whole — clone it here.
    let first = cast(spell);

    // ...so this second cast has nothing left to say.
    let second = cast(spell);

    (first, second)
}

fn main() {
    let (first, second) = practice_twice();
    println!("First casting:  {first}");
    println!("Second casting: {second}");
}

// ─── Wren's hop-test (leave this part alone) ──────────────────────
#[test]
fn both_castings_sparkle() {
    let (first, second) = practice_twice();
    assert_eq!(first, "~ double step ~");
    assert_eq!(second, "~ double step ~");
}
