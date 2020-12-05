use std::io::{stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    execute,
    style::{Color, ResetColor},
};

mod terminal_ui;

use terminal_ui::HexagonPattern;
use terminal_ui::Hexagon;
use terminal_ui::Map;

fn create_hexagon(x: u16, y: u16, foreground_color: Color) -> Hexagon {
    Hexagon {
        foreground_color: foreground_color,
        background_color: Color::Black,
        col: x,
        row: y,
    }
}

fn main() {

    let hexagon_pattern_string = include_str!("resources/big_hexagon.txt");
    let ship = include_str!("resources/map.txt");

    execute!(
        stdout(),
        Clear(ClearType::All),
    );

    let hexagon_pattern = HexagonPattern::new(hexagon_pattern_string);

    let mut map = Map {
        width: 160,
        height: 50,
        hexagons: vec![],
        hexagon_pattern: hexagon_pattern,
        map: ship,
    };

    let mut hexagons: Vec<(Hexagon, i32)> = vec!();

    for (i,s) in ship.lines().enumerate() {
        for (j,c) in s.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            let hexagon = 
                match c {
                    '0' => { create_hexagon(j as u16, i as u16, Color::Rgb{r:20, g:20, b:20}) }
                    '1' => { create_hexagon(j as u16, i as u16, Color::Blue) }
                    '2' => { create_hexagon(j as u16, i as u16, Color::Yellow) }
                    _ => { panic!("Map contains incorrect symbol") }
                };
            hexagons.push((hexagon, (c.to_string()).parse::<i32>().unwrap()));
        }
    }

    hexagons.sort_by_key(|k| k.1);

    map.hexagons = hexagons.iter().map(|k| k.0).collect();
    map.draw(0,0);

    execute!(
        stdout(),
        ResetColor,
        crossterm::cursor::MoveTo(0, 60),
    );
}
