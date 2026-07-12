// Reference solution — Quest 24: an enum's two variants, and each tide
// turning into the other.

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tide {
    Ebb,
    Flood,
}

/// The tide that follows this one. The sea has never once varied.
fn turn(tide: Tide) -> Tide {
    if tide == Tide::Ebb { Tide::Flood } else { Tide::Ebb }
}

fn main() {
    let morning = Tide::Ebb;
    let midday = turn(morning);
    println!("Morning: {morning:?}. Midday: {midday:?}.");
    println!("Evening: {:?}. The board can't lie anymore.", turn(midday));
}

#[test]
fn the_tide_turns() {
    assert_eq!(turn(Tide::Ebb), Tide::Flood);
    assert_eq!(turn(Tide::Flood), Tide::Ebb);
}

#[test]
fn two_turns_bring_it_home() {
    assert_eq!(turn(turn(Tide::Ebb)), Tide::Ebb);
}
