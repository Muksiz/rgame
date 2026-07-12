// Reference solution — Quest 27: match arms that bind the values riding
// inside their variants.

enum Signal {
    AllClear,
    Ships(u32),
    Fog(u32), // thickness, measured in gull-lengths, as is proper
}

fn read_signal(signal: Signal) -> String {
    match signal {
        Signal::AllClear => String::from("all clear"),
        Signal::Ships(count) => format!("{count} ships in the channel"),
        Signal::Fog(thickness) => format!("fog {thickness} gulls thick"),
    }
}

fn main() {
    println!("{}", read_signal(Signal::AllClear));
    println!("{}", read_signal(Signal::Ships(3)));
    println!("{}", read_signal(Signal::Fog(7)));
}

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
