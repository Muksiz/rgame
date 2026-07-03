// Reference solution — Quest 18: narrow the slice to the first three days.

fn early_week_total(week: &[u32]) -> u32 {
    let early = &week[..3];
    let mut total = 0;
    for count in early {
        total += count;
    }
    total
}

fn main() {
    let week = [2, 4, 6, 100, 100, 100, 100];
    println!("Early-week total: {}", early_week_total(&week));
}

#[test]
fn only_the_early_days_count() {
    assert_eq!(early_week_total(&[2, 4, 6, 100, 100, 100, 100]), 12);
}

#[test]
fn a_short_week_still_works() {
    assert_eq!(early_week_total(&[1, 1, 1]), 3);
}
