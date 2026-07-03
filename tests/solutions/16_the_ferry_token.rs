// Reference solution — Quest 16: borrow with &str instead of taking the String.

fn show_token(token: &str) -> String {
    format!("Wick squints at it: '{token}'. Genuine!")
}

fn crossing_paperwork() -> (String, String) {
    let token = String::from("Silverford Token No. 7");
    let receipt = show_token(&token);
    (token, receipt)
}

fn main() {
    let (token, receipt) = crossing_paperwork();
    println!("{receipt}");
    println!("Returned to your pocket: {token}");
}

#[test]
fn the_token_comes_back() {
    let (token, _) = crossing_paperwork();
    assert_eq!(token, "Silverford Token No. 7");
}

#[test]
fn the_receipt_is_written() {
    let (_, receipt) = crossing_paperwork();
    assert!(receipt.contains("Genuine"));
}
