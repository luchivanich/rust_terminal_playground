use std::io::{stdout, Write};
use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};

pub struct Hexagon {
    pub foreground_color: Color,
    pub background_color: Color,
    pub content: Vec<char>,
    pub col: u16,
    pub row: u16,
}

impl Hexagon { // TODO: rework. too slow
    fn draw(&self, pattern: &str, map_x: u16, map_y: u16) {

        let x = map_x + self.col * 6;
        let y = map_y + self.row * 4 + if self.col % 2 == 1 { 2 } else { 0 };

        execute!(
            stdout(),
            SetBackgroundColor(self.background_color),
            SetForegroundColor(self.foreground_color),
        );

        for (shift_y, line) in pattern.lines().enumerate() {
            for (shift_x, c) in line.chars().enumerate() {
                if c != 'i' {
                    execute!(
                        stdout(),
                        crossterm::cursor::MoveTo(x + shift_x as u16, y + shift_y as u16),
                        Print(c),
                    );
                }
            }
        }
    }
}

pub struct Map {
    pub width: u16,
    pub height: u16,
    pub hexagons: Vec<Hexagon>,

    pub hexagon_pattern: &'static str,
    pub map: &'static str,
}

impl Map {
    pub fn draw(&self, start_x: u16, start_y: u16) {
        for hexagon in self.hexagons.iter() {
            hexagon.draw(self.hexagon_pattern, start_x, start_y);
        }
    }
}