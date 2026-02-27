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
                    self.state.is_echo_metadata_buffer_being_filled = false;
                    return Ok(());
                }
                _ => {
                    self.state.buffer.push_str(&key_event.code.to_string());
                    return Ok(());
                }
            }
        }

        match key_event.code {
            KeyCode::Char('w') => {
                self.state.echo_metadata_selected_pos =
                    self.state.echo_metadata_selected_pos.saturating_sub(1)
            }
            KeyCode::Char('s') => {
                self.state.echo_metadata_selected_pos = self
                    .state
                    .echo_metadata_selected_pos
                    .saturating_add(1)
                    .min(8)
            }
            KeyCode::Enter => {
                if !self.state.is_echo_metadata_buffer_being_filled {
                    self.state.is_echo_metadata_buffer_being_filled =
                        !self.state.is_echo_metadata_buffer_being_filled;
                }

                let selected_song = &self.state.local_songs[self.state.selected_song_pos];
                match self.state.echo_metadata_selected_pos {
                    1 => {}
                    2 => {}
                    3 => {}
                    4 => {}
                    5 => {}
                    6 => {}
                    7 => {}
                    8 => {}
                    9 => {}
                    _ => {}
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
