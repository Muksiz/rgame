// Reference solution — Quest 2: `mut` lets the binding change.

fn loaves_by_evening() -> i32 {
    let mut loaves = 12;
    loaves = loaves + 9;
    loaves - 4
}

fn main() {
    println!("The ledger reads: {} loaves.", loaves_by_evening());
}

#[test]
fn the_ledger_adds_up() {
    assert_eq!(loaves_by_evening(), 17, "12 + 9 - 4 loaves should be 17");
}
