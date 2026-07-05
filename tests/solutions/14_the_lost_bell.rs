// Reference solution — Quest 14: a three-letter slice, taken through a
// welcoming &str parameter.

fn tonights_grounds(strip: &str) -> &str {
    &strip[..3]
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

#[test]
fn an_evening_is_three_grounds() {
    assert_eq!(plan_the_search(), "tonight: RHF");
}

#[test]
fn a_scrap_of_writing_works_too() {
    // A string literal is already a &str — a welcoming rune takes it as-is.
    assert_eq!(tonights_grounds("GPWRHF"), "GPW");
}
