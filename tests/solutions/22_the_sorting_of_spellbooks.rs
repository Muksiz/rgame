// Reference solution — Quest 22: one &mut self method; the cart counts its
// own homecomings.

struct ReturnsCart {
    waiting: u32,
    shelved: u32,
}

impl ReturnsCart {
    fn all_shelved(&self) -> bool {
        self.waiting == 0
    }

    fn shelve_one(&mut self) {
        self.waiting -= 1;
        self.shelved += 1;
    }
}

fn the_evening_sorting() -> ReturnsCart {
    let mut cart = ReturnsCart { waiting: 3, shelved: 0 };
    cart.shelve_one();
    cart.shelve_one();
    cart.shelve_one();
    cart
}

fn main() {
    let cart = the_evening_sorting();
    println!(
        "{} shelved, {} waiting. {}",
        cart.shelved,
        cart.waiting,
        if cart.all_shelved() { "The books are humming." } else { "The cart sulks on." }
    );
}

#[test]
fn every_homecoming_is_counted() {
    let cart = the_evening_sorting();
    assert_eq!(cart.shelved, 3);
    assert_eq!(cart.waiting, 0);
    assert!(cart.all_shelved());
}

#[test]
fn a_loaded_cart_knows_it() {
    let cart = ReturnsCart { waiting: 2, shelved: 0 };
    assert!(!cart.all_shelved());
}

#[test]
fn one_shelving_at_a_time() {
    let mut cart = ReturnsCart { waiting: 2, shelved: 5 };
    cart.shelve_one();
    assert_eq!(cart.waiting, 1);
    assert_eq!(cart.shelved, 6);
}
