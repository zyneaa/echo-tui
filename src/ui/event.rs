use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use super::EchoCanvas;
use crate::app::{EchoSubTab, LogLevel, Report};
use crate::awdio::AudioPlayer;
use crate::result::{EchoError, EchoResult};
use crate::{app::SelectedTab, awdio::skip, ui::AudioData};

impl EchoCanvas {
    pub async fn handle_events(&mut self, evt: Event) -> EchoResult<()> {
        let exit = match evt {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        };

        exit
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> EchoResult<()> {
        match key_event.code {
            KeyCode::Esc => {
                self.state.exit = true;
                return Ok(());
            }
            KeyCode::Right => {
                self.next_tab();
                return Ok(());
            }
            KeyCode::Left => {
                self.previous_tab();
                return Ok(());
            }
            _ => {}
        }

        match self.state.selected_tab {
            SelectedTab::Echo => self.handle_echo_key_event(key_event)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_echo_key_event(&mut self, key_event: KeyEvent) -> EchoResult<()> {
        if self.state.is_echo_metadata_buffer_being_filled {
            return self.handle_echo_metadata_key_event(key_event);
        }

        match key_event.code {
            KeyCode::Char('M') => self.state.switch_echo_subtab('M'),
            KeyCode::Char('S') => self.state.switch_echo_subtab('S'),

            KeyCode::Char('P') => self.toggle_pause()?,
            KeyCode::Char('K') => self.adjust_volume(0.1)?,
            KeyCode::Char('J') => self.adjust_volume(-0.1)?,
            KeyCode::Char('h') => self.skip_audio(-1.0)?,
            KeyCode::Char('l') => self.skip_audio(1.0)?,

            _ => match self.state.echo_subtab {
                EchoSubTab::SEARCH => return self.handle_echo_search_key_event(key_event),
                EchoSubTab::METADATA => return self.handle_echo_metadata_key_event(key_event),
                _ => {}
            },
        }
        Ok(())
    }

    fn handle_echo_search_key_event(&mut self, key_event: KeyEvent) -> EchoResult<()> {
        match key_event.code {
            KeyCode::Char('w') => self.state.previous_local_song(),
            KeyCode::Char('s') => self.state.next_local_song(),
            KeyCode::Enter => match self.state.local_songs.get(self.state.selected_song_pos) {
                Some(v) => {
                    let reporter = self.state.report_tx.clone();
                    let audio_player = match AudioPlayer::new(&v.path) {
                        Ok(player) => player,
                        Err(e) => {
                            reporter
                                .send(Report {
                                    log: Some(EchoError::LockPoisoned(e.to_string())),
                                    level: LogLevel::ERR,
                                })
                                .ok();
                            AudioPlayer::bad()
                        }
                    };
                    self.state.active_track =
                        self.state.local_songs[self.state.selected_song_pos].to_owned();
                    self.audio_player = audio_player;

                    let mut audio_state = Some(self.audio_player.state.clone());
                    if let Err(_) = self.audio_player.play() {
                        audio_state = None
                    }
                    self.audio_state = audio_state
                }
                None => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_echo_metadata_key_event(&mut self, key_event: KeyEvent) -> EchoResult<()> {
        if self.state.is_echo_metadata_buffer_being_filled {
            match key_event.code {
                KeyCode::Enter => {
                    let selected_song = &mut self.state.local_songs[self.state.selected_song_pos];
                    match self.state.echo_metadata_selected_pos {
                        0 => {
                            selected_song.metadata.title = self.state.buffer.clone();
                        }
                        1 => {
                            selected_song.metadata.artist = self.state.buffer.clone();
                        }
                        2 => {
                            selected_song.metadata.album = self.state.buffer.clone();
                        }
                        3 => {
                            selected_song.metadata.year = self
                                .state
                                .buffer
                                .parse::<u32>()
                                .unwrap_or(selected_song.metadata.year);
                        }
                        4 => {
                            selected_song.metadata.genre = self.state.buffer.clone();
                        }
                        5 => {
                            selected_song.metadata.track_number = self
                                .state
                                .buffer
                                .parse::<u32>()
                                .unwrap_or(selected_song.metadata.track_number);
                        }
                        6 => {
                            selected_song.metadata.total_tracks = self
                                .state
                                .buffer
                                .parse::<u32>()
                                .unwrap_or(selected_song.metadata.total_tracks);
                        }
                        7 => {
                            selected_song.metadata.disc_number = self
                                .state
                                .buffer
                                .parse::<u32>()
                                .unwrap_or(selected_song.metadata.disc_number);
                        }
                        8 => {
                            selected_song.metadata.total_discs = self
                                .state
                                .buffer
                                .parse::<u32>()
                                .unwrap_or(selected_song.metadata.total_discs);
                        }
                        _ => {}
                    }

                    let metadata_to_save = selected_song.metadata.clone();
                    let path_to_save = selected_song.path.clone();
                    let reporter = self.state.report_tx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = metadata_to_save.update_file(&path_to_save) {
                            let _ = reporter.send(Report {
                                log: Some(EchoError::AudioTagError(e)),
                                level: LogLevel::ERR,
                            });
                        }
                    });

                    self.state.is_echo_metadata_buffer_being_filled = false;
                    self.state.buffer = String::new();
                    return Ok(());
                }
                KeyCode::Char(c) => {
                    self.state.buffer.push(c);
                    return Ok(());
                }
                KeyCode::Backspace => {
                    self.state.buffer.pop();
                    return Ok(());
                }
                _ => return Ok(()),
            }
        }

        match key_event.code {
            KeyCode::Char('w') => {
                self.state.echo_metadata_selected_pos =
                    self.state.echo_metadata_selected_pos.saturating_sub(1)
            }
            KeyCode::Char('s') => {
                self.state.echo_metadata_selected_pos =
                    (self.state.echo_metadata_selected_pos + 1).min(8)
            }
            KeyCode::Enter => {
                self.state.is_echo_metadata_buffer_being_filled = true;
                let selected_song = &self.state.local_songs[self.state.selected_song_pos];
                let metadata = &selected_song.metadata;

                match self.state.echo_metadata_selected_pos {
                    0 => self.state.buffer = metadata.title.clone(),
                    1 => self.state.buffer = metadata.artist.clone(),
                    2 => self.state.buffer = metadata.album.clone(),
                    3 => self.state.buffer = metadata.year.to_string(),
                    4 => self.state.buffer = metadata.genre.clone(),
                    5 => self.state.buffer = metadata.track_number.to_string(),
                    6 => self.state.buffer = metadata.total_tracks.to_string(),
                    7 => self.state.buffer = metadata.disc_number.to_string(),
                    8 => self.state.buffer = metadata.total_discs.to_string(),
                    _ => self.state.buffer = String::new(),
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn skip_audio(&mut self, amount: f64) -> EchoResult<()> {
        let _ = self.with_audio_state(|state| {
            let _ = skip(state, amount);
        });

        Ok(())
    }

    fn adjust_volume(&mut self, amount: f32) -> EchoResult<()> {
        self.with_audio_state(|state| state.volume = (state.volume + amount).clamp(0.0, 1.0))
    }

    fn toggle_pause(&mut self) -> EchoResult<()> {
        self.with_audio_state(|state| state.is_pause = !state.is_pause)
    }

    fn with_audio_state<F>(&self, f: F) -> EchoResult<()>
    where
        F: FnOnce(&mut AudioData),
    {
        if let Some(audio_arc_mutex) = &self.audio_state {
            let mut state = audio_arc_mutex
                .lock()
                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
            f(&mut state);
        }

        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.state.selected_tab = self.state.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.state.selected_tab = self.state.selected_tab.previous();
    }
}
