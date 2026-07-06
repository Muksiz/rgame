// Reference solution — Quest 2: shadow `apples` with a fresh `let`, so the
// same name can turn from chalk-stroke text into its numeric count.

fn market_sign() -> String {
    let apples = "|||||||||||||||||||||";
    let slate = format!("tally: {apples}");

    let apples = apples.len();

    let pears = 7;
    format!("{slate} - {apples} apples and {pears} pears on the stall today")
}

fn main() {
    println!("{}", market_sign());
}

#[test]
fn the_sign_reads_true() {
    assert_eq!(
        market_sign(),
        "tally: ||||||||||||||||||||| - 21 apples and 7 pears on the stall today"
    );
}
