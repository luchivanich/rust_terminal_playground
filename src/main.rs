use std::io::{stdout, Write};

use crossterm::{
    terminal::{Clear, ClearType},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

enum HexagonEdge {
    Top,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Bottom
}

fn main() {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
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
    for x in 0..lenght {
        for y in 0..height {
            cur_x = x * x_shift;
            cur_y = if x % 2 == 0 { y * y_shift } else { y * y_shift + y_semi_shift };
            draw_hexagon_at(cur_x, cur_y);
        }
    }
}

fn draw_hexagon_at(x: u16, y: u16) {
    draw_hexagon_edge(HexagonEdge::Top, x, y);
    draw_hexagon_edge(HexagonEdge::TopLeft, x, y);
    draw_hexagon_edge(HexagonEdge::TopRight, x, y);
    draw_hexagon_edge(HexagonEdge::BottomLeft, x, y);
    draw_hexagon_edge(HexagonEdge::BottomRight, x, y);
    draw_hexagon_edge(HexagonEdge::Bottom, x, y);
}

fn draw_hexagon_edge(hexagon_edge: HexagonEdge, x: u16, y: u16) {
    match hexagon_edge {
        HexagonEdge::Top => {
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x + 2, y),
                Print("____"),
            );
        }
        HexagonEdge::TopLeft => {
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x+1, y+1),
                Print("/"),
                crossterm::cursor::MoveTo(x, y+2),
                Print("/"),
            );
        }
        HexagonEdge::TopRight => {
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x+6, y+1),
                Print("\\"),
                crossterm::cursor::MoveTo(x+7, y+2),
                Print("\\"),
            );
        }
        HexagonEdge::BottomLeft => {
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x, y+3),
                Print("\\"),
                crossterm::cursor::MoveTo(x+1, y+4),
                Print("\\"),
            );
        }
        HexagonEdge::BottomRight => {
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x+7, y+3),
                Print("/"),
                crossterm::cursor::MoveTo(x+6, y+4),
                Print("/"),
            );
        }
        HexagonEdge::Bottom => {
            execute!(
                stdout(),
                crossterm::cursor::MoveTo(x+2, y+4),
                Print("____"),
            );
        }
    }
}
