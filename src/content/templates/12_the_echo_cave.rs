// ══════════════════════════════════════════════════════════════════
//   Quest 12: The Echo Cave                     ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Shepherd Ambrose: "Give the rune a word and a number, and it
//   repeats the word that many times with a space between.
//   'baa baa baa'. The sheep settle right down. Bliss."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   Build the echo with a loop. Useful pieces:
//
//       let mut result = String::new();
//       for _ in 0..times { ... }
//       result.push_str(word);      result.push(' ');
//
//   Careful: single spaces BETWEEN words only — the echo is fussy
//   about trailing spaces.
//
//   Replace the `todo!()`, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn echo(word: &str, times: u32) -> String {
    todo!()
}

fn main() {
    println!("The cave answers: {}", echo("baa", 3));
}

// ─── The cave's acoustics (leave this part alone) ─────────────────
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
