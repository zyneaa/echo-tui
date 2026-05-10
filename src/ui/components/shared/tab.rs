use crate::app::SelectedTab;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use strum::IntoEnumIterator;

pub fn tabs(
    selected_tab: SelectedTab,
    block: Block<'static>,
    spinner: usize,
    animation_spinner: Vec<char>,
    fg: Color,
    accent: Color,
) -> Paragraph<'static> {
    let mut spans = vec![];
    for (i, title) in SelectedTab::iter().enumerate() {
        let is_selected = i == selected_tab as usize;
        let content = title.title();
        let span = if is_selected {
            Span::styled(
                format!(" {} {} ", content, animation_spinner[spinner]),
                Style::default().fg(fg).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                format!(" {} | ", content),
                Style::default().fg(accent).add_modifier(Modifier::DIM),
            )
        };
        spans.push(span);
    }

    Paragraph::new(Line::from(spans))
        .left_aligned()
        .block(block)
        .alignment(ratatui::layout::Alignment::Center)
}
