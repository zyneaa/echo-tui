use ratatui::{
    layout::Constraint, style::{Color, Modifier, Style}, text::Text, widgets::{Cell, Row, Table}
};

use crate::{app::EchoSubTab, awdio::song::Song, db::Playlist};

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
