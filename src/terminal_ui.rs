use std::io::{stdout, Write};
use crossterm::{
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
};

pub struct HexagonPattern {
    height: u16,
    active_width: u16,
    lines: Vec<(String, u16)>,
}

impl HexagonPattern {
    pub fn new(pattern_string: &str) -> HexagonPattern {
        let mut width = 0;
        let mut max_shift = 0;
        let mut lines: Vec<(String, u16)> = vec![];
        for s in pattern_string.lines() {
            let mut shift_x = 0;
            for c in s.chars() {
                if c == 'i' {
                    shift_x += 1;
                }
                if shift_x > max_shift {
                    max_shift = shift_x;
                }
            }
            let sub_s: String = s.chars().skip(shift_x).collect();
            if sub_s.chars().count() > width {
                width = s.chars().count();
            }

            let pair = (sub_s, shift_x as u16);
            lines.push(pair);
        }

        
        println!("Height: {}, Width: {}, Max shift: {}", lines.len(), width, max_shift);

        HexagonPattern {height: (lines.len() - 1) as u16, active_width: (width - max_shift) as u16, lines: lines}
    }
}

#[derive(Copy, Clone)]
pub struct Hexagon {
    pub foreground_color: Color,
    pub background_color: Color,
    pub col: u16,
    pub row: u16,
}

impl Hexagon { // TODO: rework. too slow
    fn draw(&self, pattern: &HexagonPattern, map_x: u16, map_y: u16) {

        let x = map_x + self.col * pattern.active_width;
        let y = map_y + self.row * pattern.height + if self.col % 2 == 1 { pattern.height / 2 } else { 0 };

        execute!(
            stdout(),
            SetBackgroundColor(self.background_color),
            SetForegroundColor(self.foreground_color),
        );

        for (i, line) in pattern.lines.iter().enumerate() {
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x + line.1, y + i as u16),
                Print(line.0.to_string()),
            );
        }
    }
}

pub struct Map {
    pub width: u16,
    pub height: u16,
    pub hexagons: Vec<Hexagon>,
    pub hexagon_pattern: HexagonPattern,
    pub map: &'static str,
}

impl Map {
    pub fn draw(&self, start_x: u16, start_y: u16) {
        for hexagon in self.hexagons.iter() {
            hexagon.draw(&self.hexagon_pattern, start_x, start_y);
        }
    }
}