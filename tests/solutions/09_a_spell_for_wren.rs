// Reference solution — Quest 9: clone into the first cast; the original
// survives for the second.

fn cast(incantation: String) -> String {
    format!("~ {incantation} ~")
}

fn practice_twice() -> (String, String) {
    let spell = String::from("double step");

    let first = cast(spell.clone());
    let second = cast(spell);

    (first, second)
}

fn main() {
    let (first, second) = practice_twice();
    println!("First casting:  {first}");
    println!("Second casting: {second}");
}

#[test]
fn both_castings_sparkle() {
    let (first, second) = practice_twice();
    assert_eq!(first, "~ double step ~");
    assert_eq!(second, "~ double step ~");
}
