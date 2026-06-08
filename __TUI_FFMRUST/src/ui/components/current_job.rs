use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    text::{Line, Span},
    widgets::{Block, BorderType, Gauge, Padding, Paragraph},
};

use crate::{app::App, ui::theme};

pub fn render(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let block = theme::card_block().padding(Padding::new(2, 2, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let vertical = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(7),
        Constraint::Min(1),
    ])
    .split(inner);

    let job = app.current_job();
    let is_failed = app.current_job_status() == crate::app::JobStatus::Failed;
    let processing = Line::from(vec![
        Span::styled(
            format!("{} ", app.spinner_frame()),
            if is_failed {
                theme::failed_style()
            } else if app.is_complete() {
                theme::done_style()
            } else {
                theme::running_style()
            },
        ),
        Span::styled(
            if is_failed {
                "Failed"
            } else if app.is_paused() {
                "Paused"
            } else if app.is_complete() {
                "Done"
            } else {
                "Processing..."
            },
            theme::primary_text_style(),
        ),
    ]);

    frame.render_widget(
        Paragraph::new("Encoding media")
            .alignment(Alignment::Center)
            .style(theme::label_style()),
        vertical[0],
    );

    frame.render_widget(
        Paragraph::new(format!("{} -> {}", job.input, job.output))
            .alignment(Alignment::Center)
            .style(theme::secondary_text_style()),
        vertical[1],
    );

    frame.render_widget(
        Paragraph::new(processing).alignment(Alignment::Center),
        vertical[2],
    );

    frame.render_widget(
        Gauge::default()
            .gauge_style(theme::gauge_fill_style())
            .ratio(app.progress_ratio())
            .label(format!("{}%", app.progress_percent()))
            .use_unicode(true),
        vertical[3],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("elapsed ", theme::muted_style()),
            Span::styled(app.elapsed_label(), theme::primary_text_style()),
            Span::raw("   "),
            Span::styled("speed ", theme::muted_style()),
            Span::styled(app.speed_label(), theme::primary_text_style()),
            Span::raw("   "),
            Span::styled("duration ", theme::muted_style()),
            Span::styled(job.duration, theme::primary_text_style()),
        ]))
        .alignment(Alignment::Center),
        vertical[4],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("preset ", theme::muted_style()),
            Span::styled(job.preset, theme::primary_text_style()),
        ]))
        .alignment(Alignment::Center),
        vertical[5],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("stage   ", theme::muted_style()),
            Span::styled(
                if is_failed {
                    "failed"
                } else if app.is_paused() {
                    "paused"
                } else if app.is_complete() {
                    "completed"
                } else {
                    job.stage
                },
                theme::primary_text_style(),
            ),
        ]))
        .alignment(Alignment::Center),
        vertical[6],
    );

    let size_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(Line::from(" size summary ").style(theme::muted_style()))
        .border_style(theme::sparkline_border_style());
    let size_inner = size_block.inner(vertical[7]);
    frame.render_widget(size_block, vertical[7]);

    let telemetry = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .margin(1)
    .split(size_inner);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("input  ", theme::muted_style()),
            Span::styled(
                format!("{} MB", job.input_size_mb),
                theme::primary_text_style(),
            ),
            Span::raw("   "),
            Span::styled("output  ", theme::muted_style()),
            Span::styled(
                if app.is_complete() {
                    format!("{} MB", job.target_output_mb)
                } else {
                    format!("~{} MB", app.output_size_mb())
                },
                theme::primary_text_style(),
            ),
        ])),
        telemetry[0],
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("saved  ", theme::muted_style()),
            Span::styled(format!("{} MB", app.saved_mb()), theme::primary_text_style()),
            Span::raw("   "),
            Span::styled("reduction  ", theme::muted_style()),
            Span::styled(
                format!("{}%", app.reduction_percent()),
                theme::primary_text_style(),
            ),
        ]))
        .alignment(Alignment::Left),
        telemetry[1],
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("target bitrate  ", theme::muted_style()),
            Span::styled(
                format!("{} kb/s", job.target_bitrate_kbps),
                theme::primary_text_style(),
            ),
        ])),
        telemetry[2],
    );

    let media_area = Layout::vertical([Constraint::Length(2), Constraint::Min(1)]).split(vertical[8]);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("codec ", theme::muted_style()),
            Span::styled(job.video_codec, theme::primary_text_style()),
            Span::raw("   "),
            Span::styled("audio ", theme::muted_style()),
            Span::styled(job.audio_codec, theme::primary_text_style()),
        ]))
        .alignment(Alignment::Center),
        media_area[0],
    );

    let bottom_text = if let Some(reason) = job.failure_reason {
        Line::from(vec![
            Span::styled("reason ", theme::muted_style()),
            Span::styled(reason, theme::failed_style()),
        ])
    } else {
        Line::from(vec![
            Span::styled("res ", theme::muted_style()),
            Span::styled(job.resolution, theme::primary_text_style()),
            Span::raw("   "),
            Span::styled("fps ", theme::muted_style()),
            Span::styled(job.fps, theme::primary_text_style()),
        ])
    };
    frame.render_widget(
        Paragraph::new(bottom_text).alignment(Alignment::Center),
        media_area[1],
    );
}
