//! The tall-grass side of the UI: wild rune encounters, and the grimoire
//! where caught runes are inscribed.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::{App, EncounterPhase};
use crate::content::wilds;
use crate::ui::{centered, panel};

const WARM: Color = Color::Rgb(238, 214, 158);
const GOLD: Color = Color::Rgb(255, 226, 150);
const DIM: Color = Color::Rgb(150, 140, 120);
const BODY: Color = Color::Rgb(214, 202, 176);
const GREEN: Color = Color::Rgb(160, 210, 140);

pub fn draw_encounter(frame: &mut Frame, rune_id: u8, selected: usize, phase: EncounterPhase) {
    let rune = wilds::wild(rune_id);
    let full = frame.area();
    let area = centered(
        full,
        66.min(full.width.saturating_sub(2)),
        16.min(full.height),
    );
    let inner = panel(frame, area, "Something stirs in the grass");

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        rune.stir,
        Style::new().fg(BODY).italic(),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("~ {} ~", rune.name),
        Style::new().fg(GOLD).bold(),
    )));
    lines.push(Line::from(""));

    match phase {
        EncounterPhase::Asking => {
            lines.push(Line::from(Span::styled(rune.prompt, Style::new().fg(WARM))));
            lines.push(Line::from(""));
            for (i, option) in rune.options.iter().enumerate() {
                let (marker, style) = if i == selected {
                    ("➤ ", Style::new().fg(GOLD).bold())
                } else {
                    ("  ", Style::new().fg(DIM))
                };
                lines.push(Line::from(Span::styled(format!("{marker}{option}"), style)));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "↑↓ choose · enter answer · esc slip away (always safe)",
                Style::new().fg(DIM),
            )));
        }
        EncounterPhase::Caught => {
            lines.push(Line::from(Span::styled(
                "The rune settles happily into your grimoire!",
                Style::new().fg(GREEN).bold(),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(rune.lore, Style::new().fg(BODY))));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "enter · back to the grass",
                Style::new().fg(DIM),
            )));
        }
        EncounterPhase::Fizzled => {
            lines.push(Line::from(Span::styled(
                "fzzt — not quite! The rune giggles and skitters off into the grass.",
                Style::new().fg(Color::Rgb(230, 170, 120)),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "No harm done. It'll rustle around here again, same question in paw.",
                Style::new().fg(DIM),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "enter · back to the grass",
                Style::new().fg(DIM),
            )));
        }
    }

    render_lines(frame, inner, lines);
}

pub fn draw_grimoire(frame: &mut Frame, app: &App) {
    let full = frame.area();
    let area = centered(
        full,
        70.min(full.width.saturating_sub(2)),
        (wilds::WILDS.len() as u16 + 9).min(full.height),
    );
    let inner = panel(frame, area, "Grimoire — wild runes of the road");

    let mut lines: Vec<Line> = Vec::new();
    for zone in 0..=3 {
        lines.push(Line::from(Span::styled(
            app.zones[zone].name,
            Style::new().fg(WARM).bold(),
        )));
        for rune in wilds::in_zone(zone) {
            if app.grimoire.contains(&rune.id) {
                lines.push(Line::from(vec![
                    Span::styled("  ✦ ", Style::new().fg(GOLD)),
                    Span::styled(rune.name, Style::new().fg(BODY).bold()),
                    Span::styled(format!("  — {}", rune.lore), Style::new().fg(DIM)),
                ]));
            } else {
                lines.push(Line::from(Span::styled(
                    "  · ???  — something still rustling out there",
                    Style::new().fg(Color::Rgb(96, 90, 78)),
                )));
            }
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!(
            "{}∕{} inscribed · wild runes live in tall grass · esc close",
            app.grimoire.len(),
            wilds::WILDS.len()
        ),
        Style::new().fg(DIM),
    )));

    render_lines(frame, inner, lines);
}

fn render_lines(frame: &mut Frame, inner: Rect, lines: Vec<Line>) {
    let para = Paragraph::new(lines).wrap(Wrap { trim: false });
    let body_area = Rect {
        x: inner.x + 2,
        y: inner.y + 1,
        width: inner.width.saturating_sub(4),
        height: inner.height.saturating_sub(2),
    };
    frame.render_widget(para, body_area);
}
