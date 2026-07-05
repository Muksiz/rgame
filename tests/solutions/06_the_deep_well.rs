// Reference solution — Quest 6: the tail expression loses its semicolon, and
// the judging-rune is a whole new function.

fn well_depth(seconds: u32) -> u32 {
    let strides = 5 * seconds * seconds;
    strides
}

fn deepest_in_the_valley(depth: u32) -> bool {
    depth > 18
}

fn main() {
    let depth = well_depth(2);
    println!("Two heartbeats down: {depth} strides!");
    println!("Deepest in the valley? {}", deepest_in_the_valley(depth));
}

#[test]
fn two_heartbeats() {
    assert_eq!(well_depth(2), 20, "5 * 2 * 2 strides");
}

#[test]
fn one_heartbeat() {
    assert_eq!(well_depth(1), 5);
}

#[test]
fn the_argument_is_settled() {
    assert!(deepest_in_the_valley(20), "twenty strides beats Millbrook's eighteen");
    assert!(!deepest_in_the_valley(18), "a tie is no victory");
    assert!(!deepest_in_the_valley(5));
}
