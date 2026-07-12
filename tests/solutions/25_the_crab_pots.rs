// Reference solution — Quest 25: enum variants carrying cargo, and
// building each shape of catch.

enum Catch {
    Nothing,
    Shrimp(u32),
    Bottle(String),
}

fn empty_pot() -> Catch {
    Catch::Nothing
}

fn shrimp_pot(count: u32) -> Catch {
    Catch::Shrimp(count)
}

fn bottle_pot(note: String) -> Catch {
    Catch::Bottle(note)
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
