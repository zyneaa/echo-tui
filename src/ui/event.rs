use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use super::EchoCanvas;
use crate::app::{EchoSubTab, LogLevel, Report};
use crate::result::{EchoError, EchoResult};
use crate::{
    app::SelectedTab,
    awdio::skip,
    ui::AudioData,
};

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
                EchoSubTab::SEARCH => {},
                EchoSubTab::METADATA => {},
                _ => {}
            },
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
