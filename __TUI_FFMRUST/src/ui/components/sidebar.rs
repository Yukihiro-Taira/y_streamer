use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{List, ListItem, Padding, Paragraph},
};

use crate::{
    app::{App, JobStatus},
    hooks::use_drag_and_drop::DragAndDropState,
    ui::theme,
};

pub fn render(frame: &mut Frame, app: &App, drag_and_drop: &DragAndDropState, area: Rect) {
    let block = theme::panel_block("Queue").padding(Padding::new(1, 1, 0, 0));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let sections = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Min(5),
        Constraint::Length(1),
        Constraint::Min(3),
    ])
    .split(inner);

    frame.render_widget(
        Paragraph::new(format!("{} / {} complete", app.completed_jobs(), app.total_jobs()))
            .style(theme::secondary_text_style()),
        sections[0],
    );

    frame.render_widget(
        Paragraph::new(format!("drop docs here: {}", app.dropped_document_count()))
            .style(theme::muted_style()),
        sections[1],
    );

    let mut items = Vec::new();
    for (index, job) in app.jobs().iter().enumerate() {
        let status = app.job_status(index);
        let marker = match status {
            JobStatus::Running => "▶",
            JobStatus::Done => "✓",
            JobStatus::Failed => "✕",
            JobStatus::Queued => "•",
        };
        let status_label = match status {
            JobStatus::Running => {
                if app.is_paused() && index == app.current_job_index() {
                    "paused".to_string()
                } else {
                    format!("{}%", app.progress_percent())
                }
            }
            JobStatus::Done => "done".to_string(),
            JobStatus::Failed => "failed".to_string(),
            JobStatus::Queued => "queued".to_string(),
        };

        let title_style = match status {
            JobStatus::Running => theme::running_style(),
            JobStatus::Done => theme::done_style(),
            JobStatus::Failed => theme::failed_style(),
            JobStatus::Queued => theme::primary_text_style(),
        };
        let status_style = match status {
            JobStatus::Running => theme::secondary_text_style(),
            JobStatus::Done => theme::done_style(),
            JobStatus::Failed => theme::failed_style(),
            JobStatus::Queued => theme::muted_style(),
        };
        let is_selected = index == drag_and_drop.selected_index();
        let is_dragging = drag_and_drop.dragging_index() == Some(index);
        let is_drop_target = drag_and_drop.drop_target_index() == Some(index);
        let row_prefix = if is_dragging {
            "↕ "
        } else if is_drop_target {
            "→ "
        } else if is_selected {
            "> "
        } else {
            "  "
        };
        let row_style = if is_dragging || is_drop_target || is_selected {
            title_style.patch(
                theme::panel_border_style()
                    .add_modifier(Modifier::BOLD)
                    .bg(ratatui::style::Color::Rgb(21, 26, 38)),
            )
        } else {
            title_style
        };
        let label_style = if is_dragging || is_drop_target {
            status_style
                .add_modifier(Modifier::BOLD)
                .bg(ratatui::style::Color::Rgb(21, 26, 38))
        } else {
            status_style
        };

        items.push(ListItem::new(Line::from(vec![
            Span::styled(row_prefix, row_style),
            Span::styled(format!("{marker} "), row_style),
            Span::styled(job.input, row_style),
            Span::raw("  "),
            Span::styled(status_label, label_style),
        ])));
    }

    frame.render_widget(List::new(items), sections[2]);

    frame.render_widget(
        Paragraph::new("documents")
            .style(theme::secondary_text_style()),
        sections[3],
    );

    let document_items: Vec<ListItem> = if app.dropped_documents().is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "  drop a file path into the terminal",
            theme::muted_style(),
        )]))]
    } else {
        app.dropped_documents()
            .iter()
            .map(|path| {
                ListItem::new(Line::from(vec![
                    Span::styled("+ ", theme::running_style()),
                    Span::styled(App::document_label(path), theme::primary_text_style()),
                ]))
            })
            .collect()
    };
    frame.render_widget(List::new(document_items), sections[4]);
}

pub fn job_list_area(area: Rect) -> Rect {
    let block = theme::panel_block("Queue").padding(Padding::new(1, 1, 0, 0));
    let inner = block.inner(area);
    let sections = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Min(5),
        Constraint::Length(1),
        Constraint::Min(3),
    ])
    .split(inner);
    sections[2]
}
