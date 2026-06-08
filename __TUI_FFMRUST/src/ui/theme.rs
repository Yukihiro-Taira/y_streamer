use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType},
};

pub fn root_block<'a>() -> Block<'a> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(Line::from(" ffmrust ").style(title_style()))
        .title_bottom(
            Line::from(" q quit ")
                .style(muted_style())
                .alignment(Alignment::Right),
        )
}

pub fn panel_block<'a>(title: &'a str) -> Block<'a> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(Line::from(format!(" {title} ")).style(label_style()))
        .border_style(panel_border_style())
}

pub fn card_block<'a>() -> Block<'a> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(panel_border_style())
}

pub fn title_style() -> Style {
    Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD)
}

pub fn label_style() -> Style {
    Style::new().fg(Color::White).add_modifier(Modifier::BOLD)
}

pub fn running_style() -> Style {
    Style::new().fg(Color::LightCyan).add_modifier(Modifier::BOLD)
}

pub fn done_style() -> Style {
    Style::new().fg(Color::Green)
}

pub fn failed_style() -> Style {
    Style::new().fg(Color::Red)
}

pub fn primary_text_style() -> Style {
    Style::new().fg(Color::White)
}

pub fn secondary_text_style() -> Style {
    Style::new().fg(Color::Gray)
}

pub fn muted_style() -> Style {
    Style::new().fg(Color::DarkGray)
}

pub fn panel_border_style() -> Style {
    Style::new().fg(Color::DarkGray)
}

pub fn sparkline_border_style() -> Style {
    Style::new().fg(Color::Rgb(44, 52, 68))
}

pub fn gauge_fill_style() -> Style {
    Style::new().fg(Color::LightCyan).bg(Color::Rgb(21, 26, 38))
}
