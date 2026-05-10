use std::path::Path;

use ratatui::{style::{Color, Modifier, Style}, symbols::border, text::{Line, Span}, widgets::{Block, Borders}};

use crate::app::EchoSubTab;

pub fn inner_input_block<'a>(
    input: &'a str,
    fg: Color,
    title_color: Color,
    echo_subtab: &EchoSubTab,
    is_focused: bool,
) -> Block<'a> {
    let block_style;
    match (echo_subtab, is_focused) {
        (EchoSubTab::IMPORT, true) => {
            block_style = Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD);
        }
        (EchoSubTab::SEARCH, true) => {
            block_style = Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD);
        }
        (_, true) => {
            block_style = Style::default().fg(fg).add_modifier(Modifier::REVERSED);
        }
        _ => {
            block_style = Style::default().fg(fg);
        }
    }

    let file_name_hint = Path::new(input)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("...");

    Block::default()
        .borders(Borders::ALL)
        .border_set(border::ROUNDED)
        .border_style(block_style)
        .title(Line::from(vec![
            Span::styled(" [ ", Style::default().fg(fg)),
            Span::styled(
                "FILE PATH",
                Style::default()
                    .fg(title_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" | {} ] ", file_name_hint), Style::default().fg(fg)),
        ]))
}

pub fn bordered_block(title: Line<'static>, color: Color) -> Block<'static> {
    Block::bordered()
        .title(title)
        .border_set(border::ROUNDED)
        .style(Style::default().fg(color))
}

pub fn unbordered_block(title: Line<'static>) -> Block<'static> {
    Block::bordered().title(title).borders(Borders::empty())
}
