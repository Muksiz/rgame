// ══════════════════════════════════════════════════════════════════
//   Quest 25: The Crab Pots                             ~ Mistholm ~
// ══════════════════════════════════════════════════════════════════
//
//   Shrimper Coble: "Every pot comes up one of three ways. Nothing.
//   Some NUMBER of shrimp. Or — once in a blue moon — a bottle with
//   a note in it. My tally board can't hold a shape like that."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Enum variants can CARRY data, each variant its own cargo:
//
//       enum Catch {
//           Nothing,
//           Shrimp(u32),        // how many
//           Bottle(String),     // what the note says
//       }
//
//   You build one with the variant's name and its cargo in
//   parentheses: `Catch::Shrimp(12)`,
//   `Catch::Bottle(String::from("hello"))` — and `Catch::Nothing`
//   plain, no parentheses, because it carries nothing at all.
//
//   1. Give Catch its three variants: Nothing, Shrimp(u32) and
//      Bottle(String).
//   2. Fix the three pot-hauling functions so each builds the
//      right kind of catch from what it's handed.
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

enum Catch {
    // TODO: three variants — two of them carrying cargo
}

fn empty_pot() -> Catch {
    todo!("no catch at all")
}

fn shrimp_pot(count: u32) -> Catch {
    todo!("a count of shrimp")
}

fn bottle_pot(note: String) -> Catch {
    todo!("a note in a bottle")
}

fn main() {
    let pots = [
        empty_pot(),
        shrimp_pot(12),
        bottle_pot(String::from("the sea says hello")),
    ];
    println!(
        "{} pots hauled, three shapes of catch. Coble nods behind the mask.",
        pots.len()
    );
}

// ─── Coble's tally board (leave this part alone) ───────────────────
#[test]
fn every_shape_of_catch_fits() {
    assert!(matches!(empty_pot(), Catch::Nothing));
    assert!(matches!(shrimp_pot(12), Catch::Shrimp(12)));
}

#[test]
fn the_note_survives_the_bottle() {
    let Catch::Bottle(note) = bottle_pot(String::from("hello from the fog")) else {
        panic!("that pot held a bottle, I'm sure of it");
    };
    assert_eq!(note, "hello from the fog");
}
