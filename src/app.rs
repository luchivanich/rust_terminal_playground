use crate::view::{draw, MapState};
use crossterm::{
    event::{Event, KeyCode, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}
};
use std::{
    error::Error,
    io::Write
};
use tui::{
    backend::CrosstermBackend,
    Terminal
};

pub struct App<W: Write> {
    terminal: Terminal<CrosstermBackend<W>>
}

impl<W: Write> App<W> {
    pub fn new(mut out: W) -> Result<Self, Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(out, EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(out))?;
        terminal.hide_cursor()?;
        Ok(Self {
            terminal
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut map_state = MapState::default();
        loop {
            self.terminal.draw(|f| draw(f, &mut map_state))?;
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => break,
                    KeyCode::Char('+') => map_state.inc_scale(),
                    KeyCode::Char('-') => map_state.dec_scale(),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }
}

impl<W: Write> Drop for App<W> {
    fn drop(&mut self) {
        self.terminal.show_cursor()
            .expect("can't show cursor");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("can't leave alternate screen");
        disable_raw_mode()
            .expect("can't disable raw mode");
    }
}
