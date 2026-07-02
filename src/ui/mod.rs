pub mod art;
pub mod cast;
pub mod dialogue;
pub mod effects;
pub mod journal;
pub mod overworld;
pub mod title;
pub mod wilds;

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Wrap};

use crate::app::{App, EPILOGUE, Screen};

/// Apply the time of day to a color: darker at night, with a soft blue cast.
/// Time of day is fixed per zone (`Zone::daylight`) — every place keeps its
/// own hour, from Emberwick's bright morning to the misty Hearthspire night.
pub fn shade(c: (u8, u8, u8), daylight: f32) -> Color {
    let bright = 0.45 + 0.55 * daylight;
    let night = 1.0 - daylight;
    let r = (c.0 as f32 * bright).min(255.0) as u8;
    let g = (c.1 as f32 * bright).min(255.0) as u8;
    let b = (c.2 as f32 * (bright + 0.22 * night)).min(255.0) as u8;
    Color::Rgb(r, g, b)
}

pub fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let w = w.min(area.width);
    let h = h.min(area.height);
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height - h) / 2,
        width: w,
        height: h,
    }
}

/// Write a string starting at absolute buffer coordinates, clipped to `area`.
pub fn put_str(buf: &mut Buffer, area: Rect, x: u16, y: u16, s: &str, style: Style) {
    if y < area.y || y >= area.y + area.height {
        return;
    }
    let mut cx = x;
    for ch in s.chars() {
        if cx < area.x || cx >= area.x + area.width {
            cx = cx.saturating_add(1);
            continue;
        }
        if let Some(cell) = buf.cell_mut((cx, y)) {
            cell.set_char(ch);
            cell.set_style(style);
        }
        cx = cx.saturating_add(1);
    }
}

/// The standard cozy overlay box: dim the world, draw a warm bordered panel.
pub fn panel(frame: &mut Frame, area: Rect, title: &str) -> Rect {
    frame.render_widget(Clear, area);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::Rgb(196, 164, 110)))
        .style(Style::new().bg(Color::Rgb(28, 24, 18)))
        .title(Line::from(vec![
            Span::styled(" ", Style::new()),
            Span::styled(
                title.to_string(),
                Style::new().fg(Color::Rgb(238, 214, 158)).bold(),
            ),
            Span::styled(" ", Style::new()),
        ]));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}

pub fn draw(frame: &mut Frame, app: &App) {
    match &app.screen {
        Screen::Title { selected } => title::draw(frame, app, *selected),
        Screen::Epilogue { page } => draw_epilogue(frame, app, *page),
        _ => {
            overworld::draw(frame, app);
            match &app.screen {
                Screen::Dialogue(d) => dialogue::draw(frame, app, d),
                Screen::Journal => journal::draw(frame, app),
                Screen::Encounter {
                    rune,
                    selected,
                    phase,
                } => wilds::draw_encounter(frame, *rune, *selected, *phase),
                Screen::Grimoire => wilds::draw_grimoire(frame, app),
                Screen::Casting { .. } => cast::draw_casting(frame, app),
                Screen::CastResult {
                    quest,
                    outcome,
                    scroll,
                } => cast::draw_result(frame, app, *quest, outcome, *scroll),
                Screen::Paused { selected } => draw_paused(frame, *selected),
                _ => {}
            }
        }
    }
}

fn draw_paused(frame: &mut Frame, selected: usize) {
    let area = centered(frame.area(), 34, 8);
    let inner = panel(frame, area, "A moment's rest");
    let items = ["Back to the road", "Save & sleep (quit)"];
    for (i, label) in items.iter().enumerate() {
        let style = if i == selected {
            Style::new().fg(Color::Rgb(255, 226, 150)).bold()
        } else {
            Style::new().fg(Color::Rgb(150, 140, 120))
        };
        let marker = if i == selected { "➤ " } else { "  " };
        let line = Paragraph::new(format!("{marker}{label}"));
        let row = Rect {
            x: inner.x + 4,
            y: inner.y + 2 + i as u16,
            width: inner.width.saturating_sub(4),
            height: 1,
        };
        frame.render_widget(line.style(style), row);
    }
}

fn draw_epilogue(frame: &mut Frame, app: &App, page: usize) {
    let full = frame.area();
    frame.render_widget(
        Block::new().style(Style::new().bg(Color::Rgb(14, 12, 20))),
        full,
    );
    effects::fireflies(frame.buffer_mut(), full, app.tick);

    let area = centered(full, 72.min(full.width), 16);
    let inner = panel(frame, area, "~ The Great Library ~");
    let text = EPILOGUE[page.min(EPILOGUE.len() - 1)];
    let para = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(Style::new().fg(Color::Rgb(226, 214, 186)));
    let body = Rect {
        x: inner.x + 2,
        y: inner.y + 1,
        width: inner.width.saturating_sub(4),
        height: inner.height.saturating_sub(3),
    };
    frame.render_widget(para, body);
    let footer = format!("Enter ▸   ({}/{})", page + 1, EPILOGUE.len());
    put_str(
        frame.buffer_mut(),
        inner,
        inner.x
            + inner
                .width
                .saturating_sub(footer.chars().count() as u16 + 2),
        inner.y + inner.height.saturating_sub(1),
        &footer,
        Style::new().fg(Color::Rgb(150, 140, 120)),
    );
}
