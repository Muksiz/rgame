//! Dev tool: render a frame of the overworld as plain text, without a TTY.
//!
//!     cargo run --example snapshot -- [zone] [x] [y] [width] [height]
//!
//! Zones 0-3 are the overworld; 4+ are the interiors behind doors (see
//! `world::zones`). Handy for eyeballing map layouts while tuning the zones.

use ratatui::Terminal;
use ratatui::backend::TestBackend;

use rgame::app::{App, Screen};
use rgame::ui;

fn main() {
    let args: Vec<i32> = std::env::args()
        .skip(1)
        .filter_map(|a| a.parse().ok())
        .collect();
    let zone = *args.first().unwrap_or(&0) as usize;
    let (w, h) = (
        *args.get(3).unwrap_or(&200) as u16,
        *args.get(4).unwrap_or(&55) as u16,
    );

    let mut app = App::new();
    app.zone_idx = zone.min(app.zones.len() - 1);
    app.player = app.zones[app.zone_idx].spawn;
    if let (Some(&x), Some(&y)) = (args.get(1), args.get(2)) {
        app.player = (x, y);
    }
    app.screen = Screen::World;
    app.tick = 1200; // animations mid-sway; time of day is fixed per zone

    let backend = TestBackend::new(w, h);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| ui::draw(frame, &app)).unwrap();

    let buffer = terminal.backend().buffer();
    for y in 0..h {
        let mut line = String::new();
        for x in 0..w {
            line.push_str(buffer[(x, y)].symbol());
        }
        println!("{}", line.trim_end());
    }
}
