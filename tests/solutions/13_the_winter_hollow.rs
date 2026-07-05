// Reference solution — Quest 13: return the owned String; ownership moves out
// to the caller and nothing dangles.

fn hoard_label(acorns: u32) -> String {
    let label = format!("{acorns} acorns against the winter");
    label
}

fn main() {
    println!("The hoard-post reads: '{}'", hoard_label(31));
}

#[test]
fn the_label_survives_the_rune() {
    assert_eq!(hoard_label(31), "31 acorns against the winter");
}

#[test]
fn even_a_lean_year_gets_its_label() {
    assert_eq!(hoard_label(3), "3 acorns against the winter");
}
