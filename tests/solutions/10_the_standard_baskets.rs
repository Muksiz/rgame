// Reference solution — Quest 10: the stamping-rune returns the basket, and
// the caller catches it — ownership in, ownership back out.

fn stamp(basket: String) -> String {
    format!("{basket} [checked]")
}

fn morning_round() -> String {
    let basket = String::from("Basket no. 3");
    stamp(basket)
}

fn main() {
    println!("Back from the stamping table: '{}'", morning_round());
}

#[test]
fn the_basket_comes_back_stamped() {
    assert_eq!(morning_round(), "Basket no. 3 [checked]");
}
