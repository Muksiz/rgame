// ══════════════════════════════════════════════════════════════════
//   Quest 12: The Echo Cave                     ~ Whispering Woods ~
// ══════════════════════════════════════════════════════════════════
//
//   Shepherd Ambrose: "My sheep settle for the night on a good
//   triple 'baa' — the call, plus two echoes on the end. My
//   echo-rune borrows the call politely, one of those `&` marks...
//   and the cave REFUSED. Looking-borrows are for looking, it says."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A shared borrow (`&String`) never permits changes. To let a
//   function CHANGE the caller's value, borrow mutably at both ends:
//
//       fn add_echo(call: &mut String) { ... }   // the pen, lent
//       add_echo(&mut call);                     // the pen, offered
//
//   Only one mutable borrow may exist at a time — nobody reads a
//   sentence while it's still being written. (That's the whole rule,
//   and the cave is very proud of it.)
//
//   Fix both ends, then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

fn add_echo(call: &String) {
    // TODO: an echo CHANGES the call — a `&` borrow won't allow it.
    call.push_str(" baa");
}

fn settle_the_sheep() -> String {
    let mut call = String::from("baa");
    add_echo(&call); // TODO: lend the pen, not just a look
    add_echo(&call);
    call
}

fn main() {
    println!("The cave answers: {}", settle_the_sheep());
}

// ─── The cave's acoustics (leave this part alone) ─────────────────
#[test]
fn a_sleepy_triple() {
    assert_eq!(settle_the_sheep(), "baa baa baa");
}
