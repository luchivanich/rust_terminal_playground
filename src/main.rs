use std::io::{stdout, Write};

use crossterm::{
    terminal::{Clear, ClearType},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};


 /* Hexagon */
trait Drawable {
    fn draw(&self);
}

struct Edge {
    foreground_color: Color,
    background_color: Color,
    edge_type: EdgeType,
    hexagon_x: u16,
    hexagon_y: u16,
}

impl Drawable for Edge {
    fn draw(&self) {
        execute!(
            stdout(),
            SetBackgroundColor(self.background_color),
            SetForegroundColor(self.foreground_color),
        );
        match self.edge_type {
            EdgeType::Top => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(self.hexagon_x + 2, self.hexagon_y),
                    Print("____"),
                );
            }
            EdgeType::TopLeft => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(self.hexagon_x+1, self.hexagon_y+1),
                    Print("/"),
                    crossterm::cursor::MoveTo(self.hexagon_x, self.hexagon_y+2),
                    Print("/"),
                );
            }
            EdgeType::TopRight => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(self.hexagon_x+6, self.hexagon_y+1),
                    Print("\\"),
                    crossterm::cursor::MoveTo(self.hexagon_x+7, self.hexagon_y+2),
                    Print("\\"),
                );
            }
            EdgeType::BottomLeft => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(self.hexagon_x, self.hexagon_y+3),
                    Print("\\"),
                    crossterm::cursor::MoveTo(self.hexagon_x+1, self.hexagon_y+4),
                    Print("\\"),
                );
            }
            EdgeType::BottomRight => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(self.hexagon_x+7, self.hexagon_y+3),
                    Print("/"),
                    crossterm::cursor::MoveTo(self.hexagon_x+6, self.hexagon_y+4),
                    Print("/"),
                );
            }
            EdgeType::Bottom => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(self.hexagon_x+2, self.hexagon_y+4),
                    Print("____"),
                );
            }
        }
    }
}

enum EdgeType {
    Top,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Bottom
}

struct Hexagon {
    edges: Vec<Edge>,
    //pub content: Vec<char>, // TODO: implement hexagon content
    pub x: u16,
    pub y: u16,
}

impl Hexagon {
    pub fn new(x: u16, y: u16 /*, edge_colors: Vec<(edge_type: EdgeType, foreground_color: Color, background_color: Color)>*/) -> Hexagon {
        let edges = vec![
            Edge{edge_type: EdgeType::Top, hexagon_x: x, hexagon_y: y, foreground_color: Color::Blue, background_color: Color::Black},
            Edge{edge_type: EdgeType::TopLeft, hexagon_x: x, hexagon_y: y, foreground_color: Color::Blue, background_color: Color::Black},
            Edge{edge_type: EdgeType::TopRight, hexagon_x: x, hexagon_y: y, foreground_color: Color::Blue, background_color: Color::Black},
            Edge{edge_type: EdgeType::BottomLeft, hexagon_x: x, hexagon_y: y, foreground_color: Color::Blue, background_color: Color::Black},
            Edge{edge_type: EdgeType::BottomRight, hexagon_x: x, hexagon_y: y, foreground_color: Color::Blue, background_color: Color::Black},
            Edge{edge_type: EdgeType::Bottom, hexagon_x: x, hexagon_y: y, foreground_color: Color::Blue, background_color: Color::Black},
        ];
        Hexagon {edges: edges, x: x, y: y}
    }
}

impl Drawable for Hexagon {
    fn draw(&self) {
        for edge in self.edges.iter() {
            edge.draw();
        }
    }
}

struct Map {
    pub width: u16,
    pub height: u16,
    pub hexagons: Vec<Hexagon>,
}

fn main() {
    execute!(
        stdout(),
        Clear(ClearType::All),
    );

    draw_grid(10,4);

    execute!(
        stdout(),
        ResetColor,
        crossterm::cursor::MoveTo(0, 30),
    );
}

fn draw_grid(lenght: u16, height: u16) {
    let y_semi_shift = 2;
    let x_shift = 6;
    let y_shift = 4;
    let mut cur_x;
    let mut cur_y;

    let mut map = Map { width: lenght, height: height, hexagons: vec!() };

    for x in 0..lenght {
        for y in 0..height {
            cur_x = x * x_shift;
            cur_y = if x % 2 == 0 { y * y_shift } else { y * y_shift + y_semi_shift };

            let hexagon = Hexagon::new(cur_x, cur_y);
            map.hexagons.push(hexagon);
        }
    }

    for h in map.hexagons.iter() {
        h.draw();
    }
}
