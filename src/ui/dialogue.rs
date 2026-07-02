use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::{App, Dialogue, DialogueKind};
use crate::ui::{art, centered, panel, put_str};

pub fn draw(frame: &mut Frame, app: &App, d: &Dialogue) {
    let full = frame.area();
    let w = full.width.saturating_sub(6).min(86);
    let h = 14.min(full.height.saturating_sub(2));
    let area = centered(
        Rect {
            x: full.x,
            y: full.y + full.height.saturating_sub(h + 1),
            width: full.width,
            height: h,
        },
        w,
        h,
    );
    let inner = panel(frame, area, &d.speaker);

    // Portrait on the left.
    let portrait = if matches!(d.kind, DialogueKind::Book) {
        art::BOOK
    } else {
        art::portrait(&d.speaker)
    };
    let portrait_lines: Vec<&str> = portrait.lines().skip_while(|l| l.is_empty()).collect();
    let portrait_w: u16 = 16;
    let buf = frame.buffer_mut();
    for (i, line) in portrait_lines
        .iter()
        .take(inner.height as usize)
        .enumerate()
    {
        put_str(
            buf,
            inner,
            inner.x + 1,
            inner.y + 1 + i as u16,
            line,
            Style::new().fg(Color::Rgb(196, 176, 140)),
        );
    }

    // Typewriter text on the right.
    let page = &d.pages[d.page.min(d.pages.len() - 1)];
    let shown: String = page.chars().take(d.revealed).collect();
    let text_area = Rect {
        x: inner.x + portrait_w + 2,
        y: inner.y + 1,
        width: inner.width.saturating_sub(portrait_w + 3),
        height: inner.height.saturating_sub(2),
    };
    let para = Paragraph::new(shown)
        .wrap(Wrap { trim: false })
        .style(Style::new().fg(Color::Rgb(228, 216, 190)));
    frame.render_widget(para, text_area);

    // Page dots + advance arrow.
    let done = d.revealed >= page.chars().count();
    let dots: String = (0..d.pages.len())
        .map(|i| if i == d.page { '●' } else { '○' })
        .collect();
    let arrow = if !done {
        "…"
    } else if d.page + 1 < d.pages.len() {
        "enter ▸"
    } else {
        "enter ✓"
    };
    let footer = format!("{dots}  {arrow}");
    let fx = inner.x
        + inner
            .width
            .saturating_sub(footer.chars().count() as u16 + 1);
    let fy = inner.y + inner.height.saturating_sub(1);
    put_str(
        frame.buffer_mut(),
        inner,
        fx,
        fy,
        &footer,
        Style::new().fg(Color::Rgb(150, 140, 120)),
    );

    let _ = app;
}
