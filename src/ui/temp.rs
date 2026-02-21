match key_event.code {
            KeyCode::Esc => self.state.exit = true,
            KeyCode::Right => self.next_tab(),
            KeyCode::Left => self.previous_tab(),

            KeyCode::Char('w') => match self.state.selected_tab {
                SelectedTab::Echo => match self.state.echo_subtab {
                    EchoSubTab::SEARCH => self.state.previous_local_song(),
                    EchoSubTab::INFO => {}
                    EchoSubTab::METADATA => {
                        self.state.echo_selected_metadata_pos =
                            self.state.echo_selected_metadata_pos.saturating_sub(1)
                    }
                },
                _ => {}
            },
            KeyCode::Char('s') => match self.state.selected_tab {
                SelectedTab::Echo => match self.state.echo_subtab {
                    EchoSubTab::SEARCH => self.state.next_local_song(),
                    EchoSubTab::INFO => {}
                    EchoSubTab::METADATA => {
                        self.state.echo_selected_metadata_pos = self
                            .state
                            .echo_selected_metadata_pos
                            .saturating_add(1)
                            .min(8);
                    }
                },
                _ => {}
            },

            KeyCode::Char('M') => match self.state.selected_tab {
                SelectedTab::Echo => {
                    self.state.switch_echo_subtab('M');
                }
                _ => {}
            },
            KeyCode::Char('S') => match self.state.selected_tab {
                SelectedTab::Echo => {
                    self.state.switch_echo_subtab('S');
                }
                _ => {}
            },

            KeyCode::Enter => match self.state.selected_tab {
                SelectedTab::Echo => match self.state.echo_subtab {
                    EchoSubTab::SEARCH => {
                        match self.state.local_songs.get(self.state.selected_song_pos) {
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
                        }
                    }
                    EchoSubTab::METADATA => {
                        let selected_song_metadata =
                            &mut self.state.local_songs[self.state.selected_song_pos].metadata;

                        match self.state.echo_selected_metadata_pos {
                            0 => {
                                selected_song_metadata.title = String::from("sx"); // Title
                            }
                            1 => {}
                            2 => {}
                            3 => {}
                            4 => {}
                            5 => {}
                            6 => {}
                            7 => {}
                            8 => {}
                            _ => {}
                        }
                    }
                    _ => {}
                },
                _ => {}
            },

            KeyCode::Char('P')
            | KeyCode::Char('K')
            | KeyCode::Char('J')
            | KeyCode::Char('h')
            | KeyCode::Char('l')
            | KeyCode::Char('H')
            | KeyCode::Char('L') => {
                if let Some(audio_arc_mutex) = &self.audio_state {
                    match key_event.code {
                        KeyCode::Char('P') => {
                            let mut state = audio_arc_mutex
                                .lock()
                                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
                            state.is_pause = !state.is_pause;
                        }
                        KeyCode::Char('K') => {
                            let mut audio = audio_arc_mutex
                                .lock()
                                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
                            audio.volume = (audio.volume + 0.1).min(1.0);
                        }
                        KeyCode::Char('J') => {
                            let mut audio = audio_arc_mutex
                                .lock()
                                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
                            audio.volume = (audio.volume - 0.1).max(0.0);
                        }
                        KeyCode::Char('h') => {
                            skip(audio_arc_mutex.clone(), -1.0)
                                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
                        }
                        KeyCode::Char('l') => {
                            skip(audio_arc_mutex.clone(), 1.0)
                                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
                        }
                        KeyCode::Char('H') => {
                            skip(audio_arc_mutex.clone(), -10.0)
                                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
                        }
                        KeyCode::Char('L') => {
                            skip(audio_arc_mutex.clone(), 10.0)
                                .map_err(|e| EchoError::LockPoisoned(e.to_string()))?;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => {
                let key = key_event.code.to_string();
                self.state.append_input(&key);
            }
        }

        Ok(())
    }


