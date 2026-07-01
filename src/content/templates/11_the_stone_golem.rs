// ══════════════════════════════════════════════════════════════════
//   Quest 11: Waking the Golem              ~ Hearthspire Approach ~
// ══════════════════════════════════════════════════════════════════
//
//   The brass plate reads: 'ADMISSIONS. Wind me thrice.'
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Methods are abilities that belong to a thing; they live in an
//   `impl` block. The golem needs three:
//
//     Golem::new()      →  a fresh golem with 0 charge
//     .wind_up()        →  adds 1 charge (changes the golem: &mut self)
//     .is_awake()       →  true at 3 or more charge (only looks: &self)
//
//   Carve all three into the impl block, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

struct Golem {
    charge: u32,
}

impl Golem {
    // TODO: fn new() -> Golem

    // TODO: fn wind_up(&mut self)

    // TODO: fn is_awake(&self) -> bool
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

// ─── The Management's acceptance test (leave this part alone) ─────
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
