// Reference solution — Quest 21: three methods in the impl block.

struct Golem {
    charge: u32,
}

impl Golem {
    fn new() -> Golem {
        Golem { charge: 0 }
    }

    fn wind_up(&mut self) {
        self.charge += 1;
    }

    fn is_awake(&self) -> bool {
        self.charge >= 3
    }
}

fn main() {
    let mut golem = Golem::new();
    golem.wind_up();
    golem.wind_up();
    golem.wind_up();
    println!(
        "The golem {}.",
        if golem.is_awake() { "grinds awake: 'ADMISSIONS. WELCOME.'" } else { "snores on" }
    );
}

#[test]
fn fresh_golems_sleep_deeply() {
    let golem = Golem::new();
    assert!(!golem.is_awake());
}

#[test]
fn two_turns_are_not_enough() {
    let mut golem = Golem::new();
    golem.wind_up();
    golem.wind_up();
    assert!(!golem.is_awake());
}

#[test]
fn thrice_wound_is_wide_awake() {
    let mut golem = Golem::new();
    golem.wind_up();
    golem.wind_up();
    golem.wind_up();
    assert!(golem.is_awake());
}
