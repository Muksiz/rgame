// Reference solution — Quest 7: build a tuple, then destructure it.

fn well_pin() -> (i32, i32) {
    (12, 4)
}

fn pin_label(pin: (i32, i32)) -> String {
    let (column, row) = pin;
    format!("column {column}, row {row}")
}

fn main() {
    println!("{}", pin_label(well_pin()));
}

#[test]
fn the_well_is_pinned() {
    assert_eq!(well_pin(), (12, 4));
}

#[test]
fn the_pin_reads_clearly() {
    assert_eq!(pin_label((12, 4)), "column 12, row 4");
}
