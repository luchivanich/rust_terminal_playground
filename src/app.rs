use crate::{
    model::Map,
    view::{draw, MapState},
};
use crossterm::{
    event::{Event, KeyCode, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}
};
use std::{
    error::Error,
    io::Write,
    rc::Rc,
};
use tui::{
    backend::CrosstermBackend,
    Terminal
};

pub struct App<W: Write> {
    map: Rc<Map>,
    terminal: Terminal<CrosstermBackend<W>>,
}

impl<W: Write> App<W> {
    pub fn new(mut out: W, map_file: &String) -> Result<Self, Box<dyn Error>> {
        let map = Rc::new(Map::load(map_file)?);
        enable_raw_mode()?;
        execute!(out, EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(out))?;
        terminal.hide_cursor()?;
        Ok(Self {
            map,
            terminal
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut map_state = MapState::new(&self.map);
        loop {
            self.terminal.draw(|f| draw(f, &mut map_state))?;
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => break,
                    KeyCode::Left => map_state.scroll_left(),
                    KeyCode::Right => map_state.scroll_right(),
                    KeyCode::Up => map_state.scroll_up(),
                    KeyCode::Down => map_state.scroll_down(),
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
