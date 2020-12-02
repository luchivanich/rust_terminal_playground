use std::io::{stdout, Write};

use crossterm::{
    terminal::{Clear, ClearType},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

#[derive(Debug, Copy, Clone)]
struct Edge {
    foreground_color: Color,
    background_color: Color,
    edge_type: EdgeType,
}

impl Edge {
    fn draw(&self, hexagon_x: u16, hexagon_y: u16) {
        execute!(
            stdout(),
            SetBackgroundColor(self.background_color),
            SetForegroundColor(self.foreground_color),
        );
        match self.edge_type {
            EdgeType::Top => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(hexagon_x + 2, hexagon_y),
                    Print("____"),
                );
            }
            EdgeType::TopLeft => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(hexagon_x+1, hexagon_y+1),
                    Print("/"),
                    crossterm::cursor::MoveTo(hexagon_x, hexagon_y+2),
                    Print("/"),
                );
            }
            EdgeType::TopRight => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(hexagon_x+6, hexagon_y+1),
                    Print("\\"),
                    crossterm::cursor::MoveTo(hexagon_x+7, hexagon_y+2),
                    Print("\\"),
                );
            }
            EdgeType::BottomLeft => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(hexagon_x, hexagon_y+3),
                    Print("\\"),
                    crossterm::cursor::MoveTo(hexagon_x+1, hexagon_y+4),
                    Print("\\"),
                );
            }
            EdgeType::BottomRight => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(hexagon_x+7, hexagon_y+3),
                    Print("/"),
                    crossterm::cursor::MoveTo(hexagon_x+6, hexagon_y+4),
                    Print("/"),
                );
            }
            EdgeType::Bottom => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(hexagon_x+2, hexagon_y+4),
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
    fn draw(&self) {
        for edge in self.edges.iter() {
            edge.draw(self.x, self.y);
        }
    }
}


// Usage: Spaceship drawing example
fn main() {
    execute!(
        stdout(),
        Clear(ClearType::All),
    );

    let ship_top_edge = Edge{edge_type: EdgeType::Top, foreground_color: Color::Red, background_color: Color::Black};
    let ship_top_left_edge = Edge{edge_type: EdgeType::TopLeft, foreground_color: Color::Red, background_color: Color::Black};
    let ship_top_right_edge = Edge{edge_type: EdgeType::TopRight, foreground_color: Color::Red, background_color: Color::Black};
    let ship_bottom_left_edge = Edge{edge_type: EdgeType::BottomLeft, foreground_color: Color::Red, background_color: Color::Black};
    let ship_bottom_right_edge = Edge{edge_type: EdgeType::BottomRight, foreground_color: Color::Red, background_color: Color::Black};
    let ship_bottom_edge = Edge{edge_type: EdgeType::Bottom, foreground_color: Color::Red, background_color: Color::Black};

    let internal_bottom_left_edge = Edge{edge_type: EdgeType::BottomLeft, foreground_color: Color::DarkGrey, background_color: Color::Black};
    let internal_bottom_right_edge = Edge{edge_type: EdgeType::BottomRight, foreground_color: Color::DarkGrey, background_color: Color::Black};
    let internal_bottom_edge = Edge{edge_type: EdgeType::Bottom, foreground_color: Color::DarkGrey, background_color: Color::Black};

    let door_top_edge = Edge{edge_type: EdgeType::Top, foreground_color: Color::Yellow, background_color: Color::Black};
    let door_top_left_edge = Edge{edge_type: EdgeType::TopLeft, foreground_color: Color::Yellow, background_color: Color::Black};
    let door_top_right_edge = Edge{edge_type: EdgeType::TopRight, foreground_color: Color::Yellow, background_color: Color::Black};
    let door_bottom_left_edge = Edge{edge_type: EdgeType::BottomLeft, foreground_color: Color::Yellow, background_color: Color::Black};
    let door_bottom_right_edge = Edge{edge_type: EdgeType::BottomRight, foreground_color: Color::Yellow, background_color: Color::Black};

    let hexagon = Hexagon {
        edges: vec![door_top_edge, ship_top_left_edge, ship_top_right_edge, ship_bottom_left_edge, ship_bottom_right_edge, internal_bottom_edge],
        x: 12,
        y: 0
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![door_top_edge, ship_top_left_edge, ship_top_right_edge, ship_bottom_left_edge, ship_bottom_right_edge, internal_bottom_edge],
        x: 24,
        y: 0
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_left_edge, ship_top_right_edge, ship_bottom_left_edge, internal_bottom_right_edge, internal_bottom_edge],
        x: 12,
        y: 4
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_left_edge, ship_top_right_edge, internal_bottom_left_edge, ship_bottom_right_edge, internal_bottom_edge],
        x: 24,
        y: 4
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_edge, internal_bottom_left_edge, internal_bottom_right_edge, internal_bottom_edge],
        x: 18,
        y: 6
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_left_edge, internal_bottom_left_edge, internal_bottom_right_edge, internal_bottom_edge],
        x: 12,
        y: 8
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_right_edge, internal_bottom_left_edge, internal_bottom_right_edge, internal_bottom_edge],
        x: 24,
        y: 8
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_edge, door_top_left_edge, ship_bottom_left_edge, internal_bottom_right_edge, internal_bottom_edge],
        x: 6,
        y: 10
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![internal_bottom_left_edge, internal_bottom_right_edge, internal_bottom_edge],
        x: 18,
        y: 10
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_edge, door_top_right_edge, ship_bottom_right_edge, internal_bottom_left_edge, internal_bottom_edge],
        x: 30,
        y: 10
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![internal_bottom_left_edge, internal_bottom_right_edge, ship_bottom_edge],
        x: 12,
        y: 12
    };
    hexagon.draw();


    let hexagon = Hexagon {
        edges: vec![internal_bottom_left_edge, internal_bottom_right_edge, ship_bottom_edge],
        x: 24,
        y: 12
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_left_edge, door_bottom_left_edge, ship_bottom_right_edge, ship_bottom_edge],
        x: 6,
        y: 14
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_bottom_left_edge, ship_bottom_right_edge, ship_bottom_edge],
        x: 18,
        y: 14
    };
    hexagon.draw();

    let hexagon = Hexagon {
        edges: vec![ship_top_right_edge, ship_bottom_left_edge, door_bottom_right_edge, ship_bottom_edge],
        x: 30,
        y: 14
    };
    hexagon.draw();

    execute!(
        stdout(),
        ResetColor,
        crossterm::cursor::MoveTo(0, 30),
    );
}
