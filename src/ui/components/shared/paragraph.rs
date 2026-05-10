use ratatui::{text::Line, widgets::{Block, Paragraph}};

pub fn paragraph(text: Vec<Line<'static>>, block: Block<'static>) -> Paragraph<'static> {
    Paragraph::new(text).block(block)
}
