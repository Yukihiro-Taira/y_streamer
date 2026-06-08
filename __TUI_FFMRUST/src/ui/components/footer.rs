use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{app::App, ui::theme};

fn key_hint<'a>(key: &'a str, label: &'a str) -> Vec<Span<'a>> {
    vec![
        Span::styled("[", theme::muted_style()),
        Span::styled(key, theme::running_style()),
        Span::styled("] ", theme::muted_style()),
        Span::styled(label, theme::secondary_text_style()),
    ]
}

pub fn render(
    frame: &mut Frame,
    app: &App,
    drop_input_label: Option<&str>,
    area: ratatui::layout::Rect,
) {
    let footer = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    frame.render_widget(
        Paragraph::new(
            drop_input_label
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("{} jobs pending", app.pending_jobs())),
        )
        .style(theme::secondary_text_style()),
        footer[0],
    );

    let mut hints: Vec<Span> = Vec::new();
    hints.extend(key_hint("enter", "confirm drop  "));
    hints.extend(key_hint("p", "pause  "));
    hints.extend(key_hint("f", "party  "));
    hints.extend(key_hint("q", "quit"));

    frame.render_widget(
        Paragraph::new(Line::from(hints)).alignment(Alignment::Right),
        footer[1],
    );
}
