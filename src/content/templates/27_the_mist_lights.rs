// ══════════════════════════════════════════════════════════════════
//   Quest 27: The Mist-Lights                           ~ Mistholm ~
// ══════════════════════════════════════════════════════════════════
//
//   Light-keeper Fathom: "ALL CLEAR is easy. But 'three ships in
//   the channel' — the THREE rides inside the signal. 'Fog seven
//   gulls thick' — the SEVEN rides inside. You must ask the signal
//   to open."
//
//   ── YOUR TASK ──────────────────────────────────────────────────
//   A match arm can reach INSIDE a variant and name what it
//   carries, right in the pattern:
//
//       match signal {
//           Signal::AllClear => String::from("all clear"),
//           Signal::Ships(count) => {
//               // from here, `count` holds the number that
//               // rode inside — ready for format!
//           }
//           ...
//       }
//
//   The book calls it *binding*: the pattern opens the cargo and
//   hands it to you by name.
//
//   Fix `read_signal` — three arms, two of them binding:
//
//       AllClear         ->  "all clear"
//       Ships(count)     ->  "3 ships in the channel"   (their count)
//       Fog(thickness)   ->  "fog 7 gulls thick"        (its thickness)
//
//   Then press `c` in the game.
// ──────────────────────────────────────────────────────────────────

enum Signal {
    AllClear,
    Ships(u32),
    Fog(u32), // thickness, measured in gull-lengths, as is proper
}

fn read_signal(signal: Signal) -> String {
    // TODO: match, binding what each signal carries
    todo!()
}

fn main() {
    println!("{}", read_signal(Signal::AllClear));
    println!("{}", read_signal(Signal::Ships(3)));
    println!("{}", read_signal(Signal::Fog(7)));
}

// ─── Fathom's reading glass (leave this part alone) ────────────────
#[test]
fn the_channel_reads_itself_aloud() {
    assert_eq!(read_signal(Signal::AllClear), "all clear");
    assert_eq!(read_signal(Signal::Ships(3)), "3 ships in the channel");
    assert_eq!(read_signal(Signal::Fog(7)), "fog 7 gulls thick");
}

#[test]
fn other_numbers_ride_just_as_well() {
    assert_eq!(read_signal(Signal::Ships(11)), "11 ships in the channel");
    assert_eq!(read_signal(Signal::Fog(1)), "fog 1 gulls thick");
}
