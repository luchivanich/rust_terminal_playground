// use std::fmt;
use std::fs;
use std::str::Lines;

use std::io::{stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    execute,
    // Result,
    // Command,
    // Ansi,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

#[derive(Debug, Copy, Clone)]
struct Edge {
    foreground_color: Color,
    background_color: Color,
    edge_type: EdgeType,
}

struct DrawEdge(pub EdgeType);

// impl fmt::Display for Ansi<EdgeType> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         Ok(())
//     }
// }

// impl Command for DrawEdge {
//     //type AnsiType = Ansi<Self>;

//     #[inline]
//     fn ansi_code(&self) -> Self::AnsiType {
//         Ansi(*self)
//     }

//     #[cfg(windows)]
//     fn execute_winapi(&self, _writer: impl FnMut() -> Result<()>) -> Result<()> {
//         Ok(())
//     }
// }

impl Edge {
    fn draw(&self, start_x: u16, start_y: u16, hexagon_x: u16, hexagon_y: u16) {

        let x = start_x + hexagon_x * 6;
        let y = start_y + hexagon_y * 4 + if hexagon_x % 2 == 1 { 2 } else { 0 };

        execute!(
            stdout(),
            SetBackgroundColor(self.background_color),
            SetForegroundColor(self.foreground_color),
        );
        match self.edge_type {
            EdgeType::Top => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x + 2, y),
                    Print("____"),
                );
            }
            EdgeType::TopLeft => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x+1, y+1),
                    Print("╱"),
                    crossterm::cursor::MoveTo(x, y+2),
                    Print("╱"),
                );
            }
            EdgeType::TopRight => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x+6, y+1),
                    Print("╲"),
                    crossterm::cursor::MoveTo(x+7, y+2),
                    Print("╲"),
                );
            }
            EdgeType::BottomLeft => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x, y+3),
                    Print("╲"),
                    crossterm::cursor::MoveTo(x+1, y+4),
                    Print("╲"),
                );
            }
            EdgeType::BottomRight => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x+7, y+3),
                    Print("╱"),
                    crossterm::cursor::MoveTo(x+6, y+4),
                    Print("╱"),
                );
            }
            EdgeType::Bottom => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(x+2, y+4),
                    Print("____"),
                );
            }
        }

        execute!(
            stdout(),
            ResetColor,
        );
    }
}

#[derive(Debug, Copy, Clone)]
enum EdgeType {
    Top,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Bottom
}

struct Hexagon {
    pub edges: Vec<Edge>,
    //pub content: Vec<char>, // TODO: implement hexagon content
    pub x: u16,
    pub y: u16,
}

impl Hexagon {
    fn draw(&self, start_x: u16, start_y: u16) {
        for edge in self.edges.iter() {
            edge.draw(start_x, start_y, self.x, self.y);
        }
    }
}

struct Map {
    pub width: u16,
    pub height: u16,
    pub hexagons: Vec<Hexagon>,
}

impl Map {
    fn draw(&self, start_x: u16, start_y: u16) {
        for hexagon in self.hexagons.iter() {
            hexagon.draw(start_x, start_y);
        }
    }
}

// Temp

fn create_ship_hexagon(x: u16, y: u16) -> Hexagon {
    let ship_top_edge = Edge{edge_type: EdgeType::Top, foreground_color: Color::Blue, background_color: Color::Black};
    let ship_top_left_edge = Edge{edge_type: EdgeType::TopLeft, foreground_color: Color::Blue, background_color: Color::Black};
    let ship_top_right_edge = Edge{edge_type: EdgeType::TopRight, foreground_color: Color::Blue, background_color: Color::Black};
    let ship_bottom_left_edge = Edge{edge_type: EdgeType::BottomLeft, foreground_color: Color::Blue, background_color: Color::Black};
    let ship_bottom_right_edge = Edge{edge_type: EdgeType::BottomRight, foreground_color: Color::Blue, background_color: Color::Black};
    let ship_bottom_edge = Edge{edge_type: EdgeType::Bottom, foreground_color: Color::Blue, background_color: Color::Black};

    Hexagon {
        edges: vec![ship_top_edge, ship_top_left_edge, ship_top_right_edge, ship_bottom_left_edge, ship_bottom_right_edge, ship_bottom_edge],
        x: x,
        y: y
    }
}

// Usage: Spaceship drawing example
fn main() {
    execute!(
        stdout(),
        Clear(ClearType::All),
    );

    let mut map = Map {
        width: 160,
        height: 50,
        hexagons: vec![],
    };

    // TODO: Move to file
    let contents = "
         00000
        0000000
        0000000
  0 0   0000000   0 0
  0 0  000000000  0 0
 000000000000000000000
 000000000000000000000
 00000 000000000 00000
        0000000
       00 000 00";

    for (i,s) in contents.lines().enumerate() {
        for (j,c) in s.chars().enumerate() {
            if c == '0' {
                map.hexagons.push(create_ship_hexagon(j as u16, i as u16));
            }
        }
    }
    map.draw(1,0);

    // let ship_top_edge = Edge{edge_type: EdgeType::Top, foreground_color: Color::Blue, background_color: Color::Black};
    // let ship_top_left_edge = Edge{edge_type: EdgeType::TopLeft, foreground_color: Color::Blue, background_color: Color::Black};
    // let ship_top_right_edge = Edge{edge_type: EdgeType::TopRight, foreground_color: Color::Blue, background_color: Color::Black};
    // let ship_bottom_left_edge = Edge{edge_type: EdgeType::BottomLeft, foreground_color: Color::Blue, background_color: Color::Black};
    // let ship_bottom_right_edge = Edge{edge_type: EdgeType::BottomRight, foreground_color: Color::Blue, background_color: Color::Black};
    // let ship_bottom_edge = Edge{edge_type: EdgeType::Bottom, foreground_color: Color::Blue, background_color: Color::Black};

    // let internal_bottom_left_edge = Edge{edge_type: EdgeType::BottomLeft, foreground_color: Color::Rgb{r:20,g:20,b:20}, background_color: Color::Black};
    // let internal_bottom_right_edge = Edge{edge_type: EdgeType::BottomRight, foreground_color: Color::Rgb{r:20,g:20,b:20}, background_color: Color::Black};
    // let internal_bottom_edge = Edge{edge_type: EdgeType::Bottom, foreground_color: Color::Rgb{r:20,g:20,b:20}, background_color: Color::Black};

    // let door_top_edge = Edge{edge_type: EdgeType::Top, foreground_color: Color::Yellow, background_color: Color::Black};
    // let door_top_left_edge = Edge{edge_type: EdgeType::TopLeft, foreground_color: Color::Yellow, background_color: Color::Black};
    // let door_top_right_edge = Edge{edge_type: EdgeType::TopRight, foreground_color: Color::Yellow, background_color: Color::Black};
    // let door_bottom_left_edge = Edge{edge_type: EdgeType::BottomLeft, foreground_color: Color::Yellow, background_color: Color::Black};
    // let door_bottom_right_edge = Edge{edge_type: EdgeType::BottomRight, foreground_color: Color::Yellow, background_color: Color::Black};

    execute!(
        stdout(),
        ResetColor,
        crossterm::cursor::MoveTo(0, 60),
    );
}
