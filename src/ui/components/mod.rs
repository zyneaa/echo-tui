use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Padding,
};

use crate::{
    app::{LogLevel, SelectedTab, State}, awdio::{AudioData, DurationInfo, current_timestamp}, config::UiConfig, ignite::Paths, ui::temp_components
};

mod tabs;
mod shared;

pub fn main_header(
    song_name_area: Rect,
    tab_area: Rc<[Rect]>,
    header_area: Rc<[Rect]>,
    body_area: Rect,
    buf: &mut Buffer,
    state: &State,
    ui_config: &UiConfig,
    audio_state: &Option<Arc<Mutex<AudioData>>>,
    all_paths: &Paths
) {
    let (
        samples,
        sample_rate,
        channels,
        file_size,
        duration,
        host,
        is_pause,
        volume,
        total_samples_played,
        min_buffer_threshold,
        fft,
        enable_fft_compute,
    ) = {
        match audio_state {
            Some(v) => {
                let audio = v.lock().unwrap();
                (
                    audio.samples.len(),
                    audio.sample_rate,
                    audio.channels,
                    audio.file_size.clone(),
                    audio.duration.clone(),
                    audio.host.clone(),
                    audio.is_pause,
                    audio.volume,
                    audio.total_samples_played,
                    audio.min_buffer_threshold,
                    audio.fft_state.clone(),
                    audio.enable_fft_compute.clone(),
                )
            }
            None => (
                0,
                0,
                0,
                String::new(),
                DurationInfo::default(),
                String::new(),
                false,
                0.0,
                0,
                0,
                Vec::new(),
                true,
            ),
        }
    };
    let max_samples = (duration.seconds * sample_rate as u64) as i64;
    let timestamp = current_timestamp(total_samples_played, sample_rate);
    let timestamp_percent = ((timestamp.1 / duration.seconds as f64) * 100.0)
        .ceil()
        .min(100.0);
    let position = (((timestamp_percent / 100.0) * 50.0) as usize).min(49);

    let text = vec![
        state.active_track.metadata.title.clone().into(),
        state.active_track.metadata.artist.clone().into(),
        "Local".into(),
    ];

    let is_playing_status = format!(
        " PLAYING: {} {} - ●  {} {}",
        !is_pause,
        ui_config.animations["animations"].hpulse[state.animations.animation_hpulse.0],
        state.echo_tab_state.is_echo_metadata_buffer_being_filled,
        state.buffer
    );
    let title_block = temp_components::bordered_block(
        Line::from(vec![Span::raw(is_playing_status)]),
        ui_config.colors["colors"].border,
    )
    .title(Line::from(" | ").right_aligned())
    .padding(Padding::horizontal(1))
    .title_bottom(Line::from(format!(" SIZE: {} ", file_size)))
    .title_bottom(Line::from(format!(" CLK: {} ", duration.readable)).right_aligned())
    .title_style(Style::new().fg(ui_config.colors["colors"].title));

    temp_components::paragraph(text, title_block)
        .style(Style::default().fg(ui_config.colors["colors"].fg))
        .render(song_name_area, buf);

    let timestamp_block =
        temp_components::bordered_block(Line::default(), ui_config.colors["colors"].border)
            .title_style(Style::new().fg(ui_config.colors["colors"].title))
            .title(Line::from(format!(" UPTIME: {} ", state.uptime_readable)).right_aligned())
            .title(Line::from(" ⟐  ").left_aligned())
            .title(Line::from(state.current_clock.clone()).centered())
            .title_bottom(Line::from(format!(" VOL: {:.1} ", volume)))
            .title_bottom(Line::from(format!(" HOST: {} ", host)).centered())
            .title_bottom(Line::from(" TICK: 100ms ").right_aligned());

    let mut anim = state.animations.animation_timestamp.borrow_mut();
    for i in 0..=anim.vals.len() - 1 {
        if i == position {
            anim.vals[position] = ui_config.animations["animations"].timestamp.clone();
            continue;
        }
        anim.vals[i] = ui_config.animations["animations"].timestamp_bar.clone();
    }
    drop(anim);

    let timestamp_bar: String = state
        .animations
        .animation_timestamp
        .borrow_mut()
        .vals
        .join("");
    let timestamp = format!(
        "{} ■ {} :|{:02}%|: {} ■ {}",
        timestamp.1 as u64, timestamp.0, timestamp_percent, duration.readable, duration.seconds
    );

    temp_components::paragraph(
        vec![Line::from(timestamp_bar), Line::from(timestamp)],
        timestamp_block,
    )
    .style(Style::default().fg(ui_config.colors["colors"].fg))
    .centered()
    .render(header_area[1], buf);

    let tab_block =
        temp_components::bordered_block(Line::default(), ui_config.colors["colors"].border)
            .title(" ● ")
            .title_style(Style::new().fg(ui_config.colors["colors"].title));

    let spinner = ui_config.animations["animations"].spinner.clone();

    temp_components::tabs(
        state.selected_tab,
        tab_block,
        state.animations.animation_spinner.0,
        spinner,
        ui_config.colors["colors"].fg,
        ui_config.colors["colors"].accent,
    )
    .render(tab_area[0], buf);

    let report = state.current_report.as_ref();
    if let Some(report) = report {
        let level = &report.level;
        if let Some(val) = &report.log {
            match level {
                LogLevel::INFO => {
                    temp_components::unbordered_block(Line::from(format!(" ⚬ {}", val)))
                        .title_style(Style::default().fg(ui_config.colors["colors"].success))
                        .render(tab_area[1], buf)
                }
                _ => {}
            }
        }
    }

    let config = ui_config;

    match state.selected_tab {
        SelectedTab::Echo => tabs::echo::render_echo(
            body_area,
            buf,
            samples,
            sample_rate,
            channels,
            total_samples_played,
            max_samples as u64,
            min_buffer_threshold,
            fft,
            ui_config.colors["colors"].fg,
            ui_config.colors["colors"].title,
            ui_config.colors["colors"].border,
            config,
            &state.local_songs,
            &state.selected_song_pos,
            &state.active_track,
            &state.echo_tab_state,
            &all_paths.songs,
        ),
        _ => {}
    }
}
