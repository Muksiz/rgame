// Reference solution — Quest 8: the label becomes a String, the keeping kind,
// and the tallies stick.

fn jar_label(caught: u32) -> String {
    let mut label = String::from("fireflies: ");
    for _ in 0..caught {
        label.push_str("*");
    }
    label
}

fn main() {
    println!("The jar reads: '{}'", jar_label(3));
}

#[test]
fn every_catch_leaves_a_mark() {
    assert_eq!(jar_label(3), "fireflies: ***");
    assert_eq!(jar_label(5), "fireflies: *****");
}

#[test]
fn an_empty_jar_is_still_labeled() {
    assert_eq!(jar_label(0), "fireflies: ");
}
