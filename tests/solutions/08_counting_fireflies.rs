// Reference solution — Quest 8: a `while` loop, one firefly at a time.

fn fill_the_jar(capacity: u32) -> u32 {
    let mut caught = 0;
    while caught < capacity {
        caught += 1;
    }
    caught
}

fn main() {
    println!("Fireflies in the jar: {}", fill_the_jar(5));
}

#[test]
fn the_jar_fills_up() {
    assert_eq!(fill_the_jar(5), 5);
}

#[test]
fn an_empty_jar_needs_no_catching() {
    assert_eq!(fill_the_jar(0), 0);
}
