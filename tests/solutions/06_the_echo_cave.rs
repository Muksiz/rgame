// Reference solution — Quest 6: a loop, a String, and careful spaces.

fn echo(word: &str, times: u32) -> String {
    let mut result = String::new();
    for _ in 0..times {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(word);
    }
    result
}

fn main() {
    println!("The cave answers: {}", echo("baa", 3));
}

#[test]
fn a_sleepy_triple() {
    assert_eq!(echo("baa", 3), "baa baa baa");
}

#[test]
fn a_single_polite_echo() {
    assert_eq!(echo("hello", 1), "hello");
}

#[test]
fn no_trailing_space() {
    assert_eq!(echo("ho", 2), "ho ho", "single spaces between, none at the end");
}
