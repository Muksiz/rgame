use ratatui::Frame;
use ratatui::style::{Color, Style};
use ratatui::widgets::Block;

use crate::app::App;
use crate::ui::{art, effects, put_str};

pub fn draw(frame: &mut Frame, app: &App, selected: usize) {
    let area = frame.area();
    frame.render_widget(
        Block::new().style(Style::new().bg(Color::Rgb(13, 15, 22))),
        area,
    );
    effects::fireflies(frame.buffer_mut(), area, app.tick);

    let logo_lines: Vec<&str> = art::LOGO.lines().filter(|l| !l.is_empty()).collect();
    let logo_w = logo_lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0) as u16;
    let use_big = area.width > logo_w + 4 && area.height > 20;
    let buf = frame.buffer_mut();

    let mut y = area.y + area.height / 6;
    if use_big {
        for line in &logo_lines {
            let x = area.x + (area.width - logo_w) / 2;
            put_str(
                buf,
                area,
                x,
                y,
                line,
                Style::new().fg(Color::Rgb(240, 205, 120)).bold(),
            );
            y += 1;
        }
    } else {
        let small = art::LOGO_SMALL.trim();
        let x = area.x + (area.width.saturating_sub(small.chars().count() as u16)) / 2;
        put_str(
            buf,
            area,
            x,
            y,
            small,
            Style::new().fg(Color::Rgb(240, 205, 120)).bold(),
        );
        y += 1;
    }

    y += 1;
    let subtitle = "a cosy little journey through the Rust programming language";
    let x = area.x + (area.width.saturating_sub(subtitle.chars().count() as u16)) / 2;
    put_str(
        buf,
        area,
        x,
        y,
        subtitle,
        Style::new().fg(Color::Rgb(150, 150, 170)).italic(),
    );

    // Ferris, keeping watch beside the menu.
    let ferris_lines: Vec<&str> = art::FERRIS.lines().filter(|l| !l.is_empty()).collect();
    for (fy, line) in (y + 3..).zip(ferris_lines.iter()) {
        let x = area.x + (area.width / 2).saturating_sub(24);
        put_str(
            buf,
            area,
            x,
            fy,
            line,
            Style::new().fg(Color::Rgb(222, 120, 80)),
        );
    }

    let items: Vec<&str> = if app.has_save {
        vec!["Continue the journey", "A new journey", "Quit"]
    } else {
        vec!["A new journey", "Quit"]
    };
    let mut my = y + 4;
    for (i, label) in items.iter().enumerate() {
        let (marker, style) = if i == selected {
            ("➤ ", Style::new().fg(Color::Rgb(255, 226, 150)).bold())
        } else {
            ("  ", Style::new().fg(Color::Rgb(140, 132, 118)))
        };
        let text = format!("{marker}{label}");
        let x = area.x + (area.width / 2).saturating_sub(4);
        put_str(buf, area, x, my, &text, style);
        my += 2;
    }

    let footer = "↑↓ choose · enter set off · you'll also want a code editor open (Zed, say) ✎";
    let x = area.x + (area.width.saturating_sub(footer.chars().count() as u16)) / 2;
    put_str(
        buf,
        area,
        x,
        area.y + area.height.saturating_sub(2),
        footer,
        Style::new().fg(Color::Rgb(110, 105, 95)),
    );
}
