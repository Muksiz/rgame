// Reference solution — Quest 2: shadow `apples` with the new count, and
// mention both fruits in the format string.

fn market_tally() -> String {
    let apples = 12;
    let apples = apples + 9;
    let pears = 7;

    format!("{apples} apples and {pears} pears on the stall today")
}

fn main() {
    println!("{}", market_tally());
}

#[test]
fn the_sign_reads_true() {
    assert_eq!(market_tally(), "21 apples and 7 pears on the stall today");
}
