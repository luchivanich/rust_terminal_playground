use std::io::{stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    execute,
    style::{Color, ResetColor},
};

mod terminal_ui;

use terminal_ui::Hexagon;
use terminal_ui::Map;

fn create_hexagon(x: u16, y: u16, foreground_color: Color) -> Hexagon {
    Hexagon {
        foreground_color: foreground_color,
        background_color: Color::Black,
        content: vec![],
        col: x,
        row: y,
    }
}

fn main() {

    let hexagon_pattern = include_str!("resources/hexagon.txt");
    let ship = include_str!("resources/map.txt");

    execute!(
        stdout(),
        Clear(ClearType::All),
    );

    let mut map = Map {
        width: 160,
        height: 50,
        hexagons: vec![],
        hexagon_pattern: hexagon_pattern,
        map: ship,
    };

    for (i,s) in ship.lines().enumerate() {
        for (j,c) in s.chars().enumerate() {
            if c == '1' {
                map.hexagons.push(create_hexagon(j as u16, i as u16, Color::Rgb{r:20, g:20, b:20}));
            }
        }
    }

    for (i,s) in ship.lines().enumerate() {
        for (j,c) in s.chars().enumerate() {
            if c == '0' {
                map.hexagons.push(create_hexagon(j as u16, i as u16, Color::Blue));
            }
        }
    }

    for (i,s) in ship.lines().enumerate() {
        for (j,c) in s.chars().enumerate() {
            if c == '2' {
                map.hexagons.push(create_hexagon(j as u16, i as u16, Color::Yellow));
            }
        }
    }

    map.draw(0,0);

    execute!(
        stdout(),
        ResetColor,
        crossterm::cursor::MoveTo(0, 60),
    );
}
