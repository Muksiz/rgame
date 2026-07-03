// Reference solution — Quest 13: an empty Vec, grown with .push().

fn gather_acorns(day1: u32, day2: u32, day3: u32) -> Vec<u32> {
    let mut hoard = Vec::new();
    hoard.push(day1);
    hoard.push(day2);
    hoard.push(day3);
    hoard
}

fn main() {
    println!("Yew's hoard: {:?}", gather_acorns(3, 5, 2));
}

#[test]
fn every_day_is_kept_in_order() {
    assert_eq!(gather_acorns(3, 5, 2), vec![3, 5, 2]);
}

#[test]
fn the_hoard_has_one_entry_per_day() {
    assert_eq!(gather_acorns(0, 0, 0).len(), 3);
}
