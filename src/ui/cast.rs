use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::App;
use crate::checker::Outcome;
use crate::content::quests::{self, FIZZLE_LINES, PASS_LINES};
use crate::ui::{art, centered, panel, put_str};
use crate::world::map::hash2;

const SPINNER: [char; 6] = ['✶', '✸', '✹', '✺', '✹', '✸'];
const WEAVING: [&str; 4] = [
    "weaving the runes…",
    "coaxing the borrow spirit…",
    "warming the type glyphs…",
    "asking rustc very nicely…",
];

pub fn draw_casting(frame: &mut Frame, app: &App) {
    let area = centered(frame.area(), 44, 7);
    let inner = panel(frame, area, "Casting");
    let spin = SPINNER[(app.tick / 2) as usize % SPINNER.len()];
    let phrase = WEAVING[(app.tick / 24) as usize % WEAVING.len()];
    let line1 = format!("{spin}  {phrase}  {spin}");
    let x = inner.x + (inner.width.saturating_sub(line1.chars().count() as u16)) / 2;
    put_str(
        frame.buffer_mut(),
        inner,
        x,
        inner.y + 2,
        &line1,
        Style::new().fg(Color::Rgb(238, 214, 158)).bold(),
    );
    let sub = "(rustc is reading your scroll)";
    let x = inner.x + (inner.width.saturating_sub(sub.chars().count() as u16)) / 2;
    put_str(
        frame.buffer_mut(),
        inner,
        x,
        inner.y + 4,
        sub,
        Style::new().fg(Color::Rgb(140, 132, 118)).italic(),
    );
}

pub fn draw_result(frame: &mut Frame, app: &App, quest_id: u8, outcome: &Outcome, scroll: u16) {
    let full = frame.area();
    match outcome {
        Outcome::Pass { .. } => draw_pass(frame, app, quest_id),
        Outcome::CompileFail { stderr } => draw_fizzle(
            frame,
            app,
            "The rune fizzles — no harm done",
            "The compiler couldn't accept the scroll yet. Its note, in full:",
            stderr,
            scroll,
        ),
        Outcome::TestFail { output } => draw_fizzle(
            frame,
            app,
            "So close! The rune sparks, but won't hold",
            "It compiles! But the quest's own judgement found something off:",
            output,
            scroll,
        ),
        Outcome::Timeout => draw_fizzle(
            frame,
            app,
            "The rune spun in circles (we stopped it gently)",
            "It ran for ten whole seconds without finishing — usually a loop that never meets its end. Check the loop's exit condition.",
            "(no output — the spell was still going in circles when Ferris unplugged it)",
            scroll,
        ),
        Outcome::Error { msg } => draw_fizzle(
            frame,
            app,
            "The satchel snags",
            "Something outside your code went wrong:",
            msg,
            scroll,
        ),
    }
    let _ = full;
}

fn draw_pass(frame: &mut Frame, app: &App, quest_id: u8) {
    let full = frame.area();
    let area = centered(full, 56.min(full.width), 15.min(full.height));
    let inner = panel(frame, area, "✦ The rune takes hold! ✦");
    let buf = frame.buffer_mut();

    let sparkle_lines: Vec<&str> = art::SPARKLES.lines().filter(|l| !l.is_empty()).collect();
    let mut y = inner.y + 1;
    for line in sparkle_lines {
        let glow = (app.tick / 4 + y as u64) % 3;
        let fg = [(255, 226, 150), (240, 196, 110), (255, 240, 190)][glow as usize];
        let x = inner.x + (inner.width.saturating_sub(line.chars().count() as u16)) / 2;
        put_str(
            buf,
            inner,
            x,
            y,
            line,
            Style::new().fg(Color::Rgb(fg.0, fg.1, fg.2)).bold(),
        );
        y += 1;
    }

    let quest = quests::quest(quest_id);
    let ferris = PASS_LINES[quest_id as usize % PASS_LINES.len()];
    let msg = format!("Quest complete: {}", quest.title);
    let x = inner.x + (inner.width.saturating_sub(msg.chars().count() as u16)) / 2;
    put_str(
        buf,
        inner,
        x,
        y + 1,
        &msg,
        Style::new().fg(Color::Rgb(170, 220, 150)).bold(),
    );
    let x = inner.x + (inner.width.saturating_sub(ferris.chars().count() as u16)) / 2;
    put_str(
        buf,
        inner,
        x.max(inner.x + 1),
        y + 2,
        ferris,
        Style::new().fg(Color::Rgb(222, 120, 80)).italic(),
    );

    let footer = "enter ▸";
    put_str(
        buf,
        inner,
        inner.x
            + inner
                .width
                .saturating_sub(footer.chars().count() as u16 + 2),
        inner.y + inner.height.saturating_sub(1),
        footer,
        Style::new().fg(Color::Rgb(150, 140, 120)),
    );
}

fn draw_fizzle(frame: &mut Frame, app: &App, title: &str, lead: &str, output: &str, scroll: u16) {
    let full = frame.area();
    let w = full.width.saturating_sub(6).min(100);
    let h = full.height.saturating_sub(4).min(30);
    let area = centered(full, w, h);
    let inner = panel(frame, area, title);

    let ferris_line =
        FIZZLE_LINES[hash2(app.tick as i32 / 600, 3, 9) as usize % FIZZLE_LINES.len()];
    let dim = Style::new().fg(Color::Rgb(150, 140, 120));

    let header = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            ferris_line,
            Style::new().fg(Color::Rgb(222, 120, 80)).italic(),
        )),
        Line::from(Span::styled(lead, dim)),
    ]))
    .wrap(Wrap { trim: false });
    let header_area = Rect {
        x: inner.x + 2,
        y: inner.y + 1,
        width: inner.width.saturating_sub(4),
        height: 3,
    };
    frame.render_widget(header, header_area);

    let body_area = Rect {
        x: inner.x + 2,
        y: inner.y + 4,
        width: inner.width.saturating_sub(4),
        height: inner.height.saturating_sub(6),
    };
    let para = Paragraph::new(output.to_string())
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0))
        .style(Style::new().fg(Color::Rgb(200, 196, 186)));
    frame.render_widget(para, body_area);

    let footer = "↑↓ scroll · enter back to the road";
    put_str(
        frame.buffer_mut(),
        inner,
        inner.x
            + inner
                .width
                .saturating_sub(footer.chars().count() as u16 + 2),
        inner.y + inner.height.saturating_sub(1),
        footer,
        dim,
    );
}
