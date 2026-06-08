use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use std::path::PathBuf;

pub struct DragAndDropState {
    selected_index: usize,
    dragging_index: Option<usize>,
    drop_target_index: Option<usize>,
}

pub enum DragAndDropAction {
    Reorder { from_index: usize, to_index: usize },
}

impl DragAndDropState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            dragging_index: None,
            drop_target_index: None,
        }
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn dragging_index(&self) -> Option<usize> {
        self.dragging_index
    }

    pub fn drop_target_index(&self) -> Option<usize> {
        self.drop_target_index
    }

    pub fn set_selected(&mut self, index: usize, item_count: usize) {
        self.selected_index = clamp_index(index, item_count);
    }

    fn move_selection_up(&mut self, item_count: usize) {
        if item_count == 0 {
            self.selected_index = 0;
            return;
        }

        self.selected_index = self.selected_index.saturating_sub(1);
    }

    fn move_selection_down(&mut self, item_count: usize) {
        if item_count == 0 {
            self.selected_index = 0;
            return;
        }

        self.selected_index = (self.selected_index + 1).min(item_count.saturating_sub(1));
    }

    fn begin_drag(&mut self, index: usize, item_count: usize) {
        if item_count == 0 {
            self.cancel_drag();
            return;
        }

        let clamped = clamp_index(index, item_count);
        self.selected_index = clamped;
        self.dragging_index = Some(clamped);
        self.drop_target_index = Some(clamped);
    }

    fn toggle_drag(&mut self, item_count: usize) -> Option<DragAndDropAction> {
        if let Some(from_index) = self.dragging_index {
            let to_index = self.drop_target_index.unwrap_or(from_index);
            self.finish_drop(item_count);
            Some(DragAndDropAction::Reorder {
                from_index,
                to_index,
            })
        } else {
            self.begin_drag(self.selected_index, item_count);
            None
        }
    }

    fn update_drop_target(&mut self, index: usize, item_count: usize) {
        if self.dragging_index.is_none() || item_count == 0 {
            return;
        }

        let clamped = clamp_index(index, item_count);
        self.selected_index = clamped;
        self.drop_target_index = Some(clamped);
    }

    fn handle_drag_motion(&mut self, kind: MouseEventKind, row_index: Option<usize>, item_count: usize) {
        match kind {
            MouseEventKind::Down(_) => {
                if let Some(index) = row_index {
                    self.begin_drag(index, item_count);
                }
            }
            MouseEventKind::Drag(_) => {
                if let Some(index) = row_index {
                    self.update_drop_target(index, item_count);
                }
            }
            MouseEventKind::Up(_) => {
                if let Some(index) = row_index {
                    self.update_drop_target(index, item_count);
                }
            }
            _ => {}
        }
    }

    fn finish_drop(&mut self, item_count: usize) {
        let fallback = self.dragging_index.unwrap_or(self.selected_index);
        self.selected_index = clamp_index(self.drop_target_index.unwrap_or(fallback), item_count);
        self.dragging_index = None;
        self.drop_target_index = None;
    }

    pub fn cancel_drag(&mut self) {
        self.dragging_index = None;
        self.drop_target_index = None;
    }

    pub fn handle_key(&mut self, key_code: KeyCode, item_count: usize) -> Option<DragAndDropAction> {
        match key_code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_selection_up(item_count);
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_selection_down(item_count);
                None
            }
            KeyCode::Char(' ') => self.toggle_drag(item_count),
            KeyCode::Esc => {
                self.cancel_drag();
                None
            }
            _ => None,
        }
    }

    pub fn handle_mouse(
        &mut self,
        mouse: MouseEvent,
        sidebar_area: Rect,
        item_count: usize,
    ) -> Option<DragAndDropAction> {
        let row_index = job_index_at(sidebar_area, mouse.column, mouse.row, item_count);

        match mouse.kind {
            MouseEventKind::Down(_) => {
                if let Some(index) = row_index {
                    self.set_selected(index, item_count);
                    self.begin_drag(index, item_count);
                }
                None
            }
            MouseEventKind::Drag(_) => {
                self.handle_drag_motion(mouse.kind, row_index, item_count);
                None
            }
            MouseEventKind::Up(_) => {
                self.handle_drag_motion(mouse.kind, row_index, item_count);
                let action = match (self.dragging_index, self.drop_target_index) {
                    (Some(from_index), Some(to_index)) => {
                        Some(DragAndDropAction::Reorder { from_index, to_index })
                    }
                    _ => None,
                };
                self.finish_drop(item_count);
                action
            }
            _ => None,
        }
    }
}

fn clamp_index(index: usize, item_count: usize) -> usize {
    if item_count == 0 {
        0
    } else {
        index.min(item_count.saturating_sub(1))
    }
}

fn job_index_at(area: Rect, column: u16, row: u16, item_count: usize) -> Option<usize> {
    if item_count == 0 {
        return None;
    }

    let list_right = area.x + area.width;
    let list_bottom = area.y + area.height;

    if column < area.x || column >= list_right || row < area.y || row >= list_bottom {
        return None;
    }

    let index = row.saturating_sub(area.y) as usize;
    if index < item_count { Some(index) } else { None }
}

pub fn parse_dropped_paths(paste: &str) -> Vec<PathBuf> {
    paste
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(normalize_dropped_path)
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .collect()
}

fn normalize_dropped_path(raw: &str) -> String {
    let trimmed = raw.trim().trim_matches(|c| c == '"' || c == '\'');
    let mut normalized = String::with_capacity(trimmed.len());
    let mut chars = trimmed.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                normalized.push(next);
            }
        } else {
            normalized.push(ch);
        }
    }

    normalized
}
