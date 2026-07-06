// Reference solution — Quest 7: a tuple for the well pin, and a `for` loop
// over an array to sum the road's legs.

fn well_pin() -> (i32, i32) {
    (12, 4)
}

fn road_length(legs: [i32; 4]) -> i32 {
    let mut total = 0;
    for leg in legs {
        total += leg;
    }
    total
}

fn main() {
    let pin = well_pin();
    // A tuple is taken apart by position — `pin.0` is the column, `pin.1` the row.
    println!("The well is pinned at column {}, row {}.", pin.0, pin.1);
    println!("The east road runs {} paces.", road_length([3, 7, 2, 5]));
}

// ─── Reed checks every pin against the map (leave this part alone) ─
#[test]
fn the_well_is_pinned() {
    assert_eq!(well_pin(), (12, 4));
}

#[test]
fn the_road_sums_true() {
    assert_eq!(road_length([3, 7, 2, 5]), 17);
    assert_eq!(road_length([1, 1, 1, 1]), 4);
}
