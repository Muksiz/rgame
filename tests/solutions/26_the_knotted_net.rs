// Reference solution — Quest 26: an exhaustive match, one arm per knot.

enum Knot {
    Reef,
    Bowline,
    Granny, // everyone ties one eventually; nobody admits it
}

fn mending_for(knot: Knot) -> String {
    match knot {
        Knot::Reef => String::from("tighten both loops"),
        Knot::Bowline => String::from("re-thread the rabbit hole"),
        Knot::Granny => String::from("cut it out and start over"),
    }
}

fn main() {
    println!("A reef knot? {}", mending_for(Knot::Reef));
    println!("A bowline? {}", mending_for(Knot::Bowline));
    println!("A... granny knot? {}", mending_for(Knot::Granny));
}

#[test]
fn every_knot_has_its_mending() {
    assert_eq!(mending_for(Knot::Reef), "tighten both loops");
    assert_eq!(mending_for(Knot::Bowline), "re-thread the rabbit hole");
    assert_eq!(mending_for(Knot::Granny), "cut it out and start over");
}
