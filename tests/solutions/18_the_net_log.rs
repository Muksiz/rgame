// Reference solution — Quest 18: a tuple struct, built like a call, read by
// position.

struct EarlyWeek(u32, u32, u32);

fn read_chart() -> EarlyWeek {
    EarlyWeek(2, 4, 6)
}

fn early_total(days: EarlyWeek) -> u32 {
    days.0 + days.1 + days.2
}

fn main() {
    let days = read_chart();
    println!("Tuesday's count: {}", days.1);
    println!("Early-week total: {}", early_total(read_chart()));
}

#[test]
fn the_early_week_is_bundled() {
    let days = read_chart();
    assert_eq!(days.0, 2);
    assert_eq!(days.1, 4);
    assert_eq!(days.2, 6);
}

#[test]
fn the_tally_reads_twelve() {
    assert_eq!(early_total(read_chart()), 12);
}
