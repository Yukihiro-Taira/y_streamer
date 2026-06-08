mod hooks;

use std::io;

use anyhow::Result;
use crossterm::event::{
    self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

use crate::hooks::use_drag_and_drop::{DragAndDropState, ProbeState};

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    let result = run_app(&mut terminal);
    restore_terminal()?;
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut drag_and_drop = DragAndDropState::default();

    loop {
        terminal.draw(|frame| render(frame, &drag_and_drop))?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Enter => drag_and_drop.commit_input(),
                KeyCode::Backspace => drag_and_drop.pop_input(),
                KeyCode::Char(ch) => drag_and_drop.push_input(ch),
                _ => {}
            },
            Event::Paste(data) => drag_and_drop.replace_input(data),
            _ => {}
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, drag_and_drop: &DragAndDropState) {
    let area = frame.area();
    let root = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" drop_video_metadata ")
        .title_bottom(Line::from(" q quit ").right_aligned());
    let inner = root.inner(area);
    frame.render_widget(root, area);

    let layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(4),
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .margin(1)
    .split(inner);

    frame.render_widget(
        Paragraph::new("Drop or paste a file path, or type one then press Enter.")
            .style(Style::new().fg(Color::Gray)),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(drag_and_drop.input())
            .block(Block::bordered().title("input"))
            .wrap(Wrap { trim: false }),
        layout[1],
    );

    let result_line = match drag_and_drop.probe_state() {
        ProbeState::Waiting => Line::from(vec![
            Span::styled(
                "status: ",
                Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw("waiting for a dropped path"),
        ]),
        ProbeState::Missing(path) => Line::from(vec![
            Span::styled(
                "missing: ",
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw(path),
        ]),
        ProbeState::ProbeError(message) => Line::from(vec![
            Span::styled(
                "ffprobe error: ",
                Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(message),
        ]),
        ProbeState::Loaded(report) => Line::from(vec![
            Span::styled(
                "loaded: ",
                Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::raw(report.display_name()),
        ]),
    };
    frame.render_widget(
        Paragraph::new(result_line).block(Block::bordered().title("status")),
        layout[2],
    );

    frame.render_widget(
        Paragraph::new(drag_and_drop.report_lines())
            .block(Block::bordered().title("metadata"))
            .wrap(Wrap { trim: false }),
        layout[3],
    );
}

fn init_terminal() -> Result<DefaultTerminal> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableBracketedPaste)?;
    Ok(ratatui::init())
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableBracketedPaste)?;
    ratatui::restore();
    Ok(())
}
