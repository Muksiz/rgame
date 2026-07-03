// Reference solution — Quest 10: array indices start at 0, so the third
// basket is index 2.

fn basket_capacities() -> [u32; 4] {
    [5, 6, 7, 8]
}

fn third_basket_capacity() -> u32 {
    let baskets = basket_capacities();
    baskets[2]
}

fn main() {
    println!("The third basket holds {}.", third_basket_capacity());
}

#[test]
fn there_are_four_standard_baskets() {
    assert_eq!(basket_capacities(), [5, 6, 7, 8]);
}

#[test]
fn the_third_basket_is_the_third() {
    assert_eq!(third_basket_capacity(), 7);
}
