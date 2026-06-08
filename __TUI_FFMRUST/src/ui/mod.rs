mod components;
mod theme;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Padding,
};

use crate::app::App;
use crate::hooks::use_drag_and_drop::DragAndDropState;

pub fn render(
    frame: &mut Frame,
    app: &App,
    drag_and_drop: &DragAndDropState,
    drop_input_label: Option<String>,
) {
    let area = frame.area();

    if app.should_celebrate() {
        components::celebration::render(frame, app, area);
        return;
    }

    let root = theme::root_block().padding(Padding::new(1, 1, 0, 0));
    let inner = root.inner(area);
    frame.render_widget(root, area);

    let shell = Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).split(inner);
    let content = Layout::horizontal([Constraint::Percentage(28), Constraint::Percentage(72)])
        .split(shell[0]);

    components::sidebar::render(frame, app, drag_and_drop, content[0]);
    components::current_job::render(frame, app, content[1]);
    components::footer::render(frame, app, drop_input_label.as_deref(), shell[1]);
}

pub fn sidebar_job_list_area(area: Rect) -> Rect {
    let root = theme::root_block().padding(Padding::new(1, 1, 0, 0));
    let inner = root.inner(area);
    let shell = Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).split(inner);
    let content = Layout::horizontal([Constraint::Percentage(28), Constraint::Percentage(72)])
        .split(shell[0]);

    components::sidebar::job_list_area(content[0])
}
