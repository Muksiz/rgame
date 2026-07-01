use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::App;
use crate::checker;
use crate::ui::{centered, panel};

pub fn draw(frame: &mut Frame, app: &App) {
    let full = frame.area();
    let area = centered(
        full,
        74.min(full.width.saturating_sub(4)),
        20.min(full.height),
    );
    let inner = panel(frame, area, "Journal");

    let mut lines: Vec<Line> = Vec::new();
    let dim = Style::new().fg(Color::Rgb(150, 140, 120));
    let warm = Style::new().fg(Color::Rgb(238, 214, 158));
    let body = Style::new().fg(Color::Rgb(214, 202, 176));

    match app.active_quest() {
        None => {
            lines.push(Line::from(Span::styled(
                "The journey is complete. ◆",
                warm.bold(),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Every rune cast, every quest done. The road stays open for wandering, and the Library has excellent armchairs.",
                body,
            )));
        }
        Some(quest) => {
            lines.push(Line::from(vec![
                Span::styled(format!("Quest {} — {}", quest.id, quest.title), warm.bold()),
                Span::styled(format!("   ({})", quest.lesson), dim.italic()),
            ]));
            lines.push(Line::from(Span::styled(
                format!("{} · {}", quest.npc, app.zones[quest.zone].name),
                dim,
            )));
            lines.push(Line::from(""));
            if app.accepted.contains(&quest.id) {
                lines.push(Line::from(Span::styled(quest.reminder, body)));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Your scroll: ", dim),
                    Span::styled(
                        format!("{}/{}", checker::QUEST_DIR, quest.file_name),
                        Style::new().fg(Color::Rgb(150, 200, 160)).bold(),
                    ),
                    Span::styled("  — edit it, then press c in the game.", dim),
                ]));
                lines.push(Line::from(""));
                let revealed = app.hints.get(&quest.id).copied().unwrap_or(0);
                if revealed == 0 {
                    lines.push(Line::from(Span::styled(
                        "Ferris is napping in your satchel. (f — ask for a hint)",
                        dim.italic(),
                    )));
                } else {
                    lines.push(Line::from(Span::styled("Ferris's hints:", warm)));
                    for hint in quest.hints.iter().take(revealed) {
                        lines.push(Line::from(vec![
                            Span::styled("  ☘ ", Style::new().fg(Color::Rgb(222, 120, 80))),
                            Span::styled(*hint, body),
                        ]));
                    }
                    if revealed < quest.hints.len() {
                        lines.push(Line::from(Span::styled(
                            format!("  ({} more — press f)", quest.hints.len() - revealed),
                            dim.italic(),
                        )));
                    }
                }
            } else {
                lines.push(Line::from(Span::styled(
                    format!(
                        "Someone needs you: find {} in {}. (They'll be under a bobbing '!')",
                        quest.npc, app.zones[quest.zone].name
                    ),
                    body,
                )));
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("Runes mastered: {}∕12    esc close", app.completed.len()),
        dim,
    )));

    let para = Paragraph::new(lines).wrap(Wrap { trim: false });
    let body_area = Rect {
        x: inner.x + 2,
        y: inner.y + 1,
        width: inner.width.saturating_sub(4),
        height: inner.height.saturating_sub(2),
    };
    frame.render_widget(para, body_area);
}
