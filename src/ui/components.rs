use std::path::Path;

use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use strum::IntoEnumIterator;
// use tracing::{debug, error, info, trace, warn};

use crate::{
    app::{EchoSubTab, SelectedTab},
    awdio::song::Song,
    db::Playlist,
};

pub fn bordered_block(title: Line<'static>, color: Color) -> Block<'static> {
    Block::bordered()
        .title(title)
        .border_set(border::ROUNDED)
        .style(Style::default().fg(color))
}

pub fn unbordered_block(title: Line<'static>) -> Block<'static> {
    Block::bordered().title(title).borders(Borders::empty())
}

pub fn paragraph(text: Vec<Line<'static>>, block: Block<'static>) -> Paragraph<'static> {
    Paragraph::new(text).block(block)
}

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

pub fn echo_metadata_table<'a>(
    metadata: Vec<(&'a str, &'a String)>,
    echo_selected_metadata_pos: usize,
    echo_subtab: &EchoSubTab,
    title: Color,
    fg: Color,
) -> Table<'a> {
    let selected_matadata_style;
    match echo_subtab {
        EchoSubTab::METADATA => {
            selected_matadata_style = Style::default().add_modifier(Modifier::REVERSED).fg(title);
        }
        _ => selected_matadata_style = Style::default().fg(fg),
    }

    let rows = metadata.iter().enumerate().map(|(i, data)| {
        let is_selected = i == echo_selected_metadata_pos;
        let row_style = if is_selected {
            selected_matadata_style
        } else {
            Style::default().fg(fg)
        };

        let (desc, val) = (data.0, data.1);

        Row::new(vec![
            Cell::from(Text::from(desc)),
            Cell::from(Text::from(val.clone())),
        ])
        .height(1)
        .style(row_style)
    });

    Table::new(
        rows,
        [Constraint::Percentage(30), Constraint::Percentage(70)],
    )
    .row_highlight_style(selected_matadata_style)
}

pub fn local_songs_table(
    songs: &Vec<Song>,
    fg: Color,
    _bg: Color,
    _accent: Color,
    title: Color,
    selected_song_pos: &usize,
    echo_subtab: &EchoSubTab,
) -> Table<'static> {
    let selected_row_style;
    match echo_subtab {
        EchoSubTab::SEARCH => {
            selected_row_style = Style::default().add_modifier(Modifier::REVERSED).fg(title);
        }
        _ => selected_row_style = Style::default(),
    }

    let rows = songs.iter().enumerate().map(|(i, data)| {
        let is_selected = i == *selected_song_pos;
        let row_style = if is_selected {
            selected_row_style
        } else {
            Style::default().fg(fg)
        };

        let item = data.ref_array();
        let (name, artist) = (item[0], item[1]);

        Row::new(vec![
            Cell::from(Text::from(name.clone())),
            Cell::from(Text::from(artist.clone())),
        ])
        .height(1)
        .style(row_style)
    });

    Table::new(
        rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .row_highlight_style(selected_row_style)
}

pub fn playlist_list_table(
    playlists: &[Playlist],
    selected_idx: usize,
    is_active: bool,
    fg: Color,
    title: Color,
) -> Table<'static> {
    let selected_style = if is_active {
        Style::default().add_modifier(Modifier::REVERSED).fg(title)
    } else {
        Style::default().fg(fg)
    };

    let rows = playlists.iter().enumerate().map(|(i, pl)| {
        let row_style = if i == selected_idx {
            selected_style
        } else {
            Style::default().fg(fg)
        };

        Row::new(vec![Cell::from(Text::from(format!(" {}", pl.name)))])
            .height(1)
            .style(row_style)
    });

    Table::new(rows, [Constraint::Percentage(100)]).row_highlight_style(selected_style)
}

pub fn playlist_songs_table(
    songs: &[Song],
    selected_idx: usize,
    is_active: bool,
    fg: Color,
    title: Color,
) -> Table<'static> {
    let selected_style = if is_active {
        Style::default().add_modifier(Modifier::REVERSED).fg(title)
    } else {
        Style::default().fg(fg)
    };

    let rows = songs.iter().enumerate().map(|(i, song)| {
        let row_style = if i == selected_idx {
            selected_style
        } else {
            Style::default().fg(fg)
        };

        Row::new(vec![
            Cell::from(Text::from(song.metadata.title.clone())),
            Cell::from(Text::from(song.metadata.artist.clone())),
        ])
        .height(1)
        .style(row_style)
    });

    Table::new(
        rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .row_highlight_style(selected_style)
}

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
