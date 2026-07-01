// Reference solution — Quest 3: convert with `as f64` before multiplying.

fn well_depth(seconds: i32) -> f64 {
    let t = seconds as f64;
    0.5 * 9.8 * t * t
}

fn main() {
    println!("Two heartbeats down: {} strides deep!", well_depth(2));
}

#[test]
fn two_heartbeats() {
    assert!((well_depth(2) - 19.6).abs() < 0.01, "two heartbeats ≈ 19.6");
}

#[test]
fn one_heartbeat() {
    assert!((well_depth(1) - 4.9).abs() < 0.01, "one heartbeat ≈ 4.9");
}
