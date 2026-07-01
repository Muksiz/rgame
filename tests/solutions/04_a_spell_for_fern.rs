// Reference solution — Quest 4: a whole function, in and out.

fn double_step(paces: i32) -> i32 {
    paces * 2
}

fn main() {
    println!("Seven paces become {}!", double_step(7));
}

#[test]
fn small_hops() {
    assert_eq!(double_step(3), 6);
}

#[test]
fn a_grand_journey() {
    assert_eq!(double_step(21), 42);
}
