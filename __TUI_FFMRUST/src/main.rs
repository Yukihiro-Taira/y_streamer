mod app;
mod hooks;
mod job;
mod terminal;
mod ui;

use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::DefaultTerminal;

use crate::app::App;
use crate::hooks::use_drag_and_drop::{
    DragAndDropAction, DragAndDropState, parse_dropped_paths,
};

const TICK_RATE: Duration = Duration::from_millis(100);

fn main() -> Result<()> {
    let mut terminal = terminal::init()?;
    let result = run_app(&mut terminal);
    terminal::restore()?;
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    let mut drag_and_drop = DragAndDropState::new();
    let mut pending_drop_input = PendingDropInput::default();

    loop {
        app.update();
        pending_drop_input.flush_if_idle(&mut app);
        terminal.draw(|frame| ui::render(frame, &app, &drag_and_drop, pending_drop_input.label()))?;

        if event::poll(TICK_RATE)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    code if pending_drop_input.handle_key(code, &mut app) => {}
                    KeyCode::Char('q') => break,
                    KeyCode::Char('f') => app.trigger_celebration(),
                    KeyCode::Char('p') => app.toggle_pause(),
                    _ => {
                        let total_jobs = app.total_jobs();
                        apply_drag_action(
                            &mut app,
                            drag_and_drop.handle_key(key.code, total_jobs),
                        );
                    }
                }
                Event::Mouse(mouse) => {
                    let size = terminal.size()?;
                    let frame_area = Rect::new(0, 0, size.width, size.height);
                    let sidebar_area = ui::sidebar_job_list_area(frame_area);
                    let total_jobs = app.total_jobs();
                    apply_drag_action(
                        &mut app,
                        drag_and_drop.handle_mouse(mouse, sidebar_area, total_jobs),
                    );
                }
                Event::Paste(data) => {
                    for path in parse_dropped_paths(&data) {
                        app.add_dropped_document(path);
                    }
                    pending_drop_input.clear();
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn apply_drag_action(app: &mut App, action: Option<DragAndDropAction>) {
    if let Some(DragAndDropAction::Reorder {
        from_index,
        to_index,
    }) = action
    {
        app.move_job(from_index, to_index);
    }
}

#[derive(Default)]
struct PendingDropInput {
    buffer: String,
    last_input_at: Option<Instant>,
}

impl PendingDropInput {
    fn handle_key(&mut self, code: KeyCode, app: &mut App) -> bool {
        match code {
            KeyCode::Enter => {
                if self.buffer.is_empty() {
                    return false;
                }
                self.flush(app);
                true
            }
            KeyCode::Backspace => {
                if self.buffer.is_empty() {
                    return false;
                }
                self.buffer.pop();
                self.last_input_at = Some(Instant::now());
                true
            }
            KeyCode::Esc => {
                if self.buffer.is_empty() {
                    return false;
                }
                self.clear();
                true
            }
            KeyCode::Char(ch) => {
                if self.buffer.is_empty() && !starts_drop_capture(ch) {
                    return false;
                }
                self.buffer.push(ch);
                self.last_input_at = Some(Instant::now());
                true
            }
            _ => !self.buffer.is_empty(),
        }
    }

    fn flush_if_idle(&mut self, app: &mut App) {
        if self.buffer.is_empty() {
            return;
        }

        let Some(last_input_at) = self.last_input_at else {
            return;
        };

        if last_input_at.elapsed() >= Duration::from_millis(350) {
            self.flush(app);
        }
    }

    fn flush(&mut self, app: &mut App) {
        for path in parse_dropped_paths(&self.buffer) {
            app.add_dropped_document(path);
        }
        self.clear();
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.last_input_at = None;
    }

    fn label(&self) -> Option<String> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(format!("drop path: {}", self.buffer))
        }
    }
}

fn starts_drop_capture(ch: char) -> bool {
    matches!(ch, '/' | '~' | '.' | '"' | '\'')
}
