use crate::model::Map;
use std::{
    cmp::{max,min},
    rc::Rc,
};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

const MAX_SCALE: i32 = 5;
const MIN_SCALE: i32 = 1;
const SHIFT_X: i32 = 10;
const SHIFT_Y: i32 = 3;

pub struct MapState {
    styles: Vec<(char, Style)>,
    scale: i32,
    shift_x: i32,
    shift_y: i32,
    map: Rc<Map>,
}

impl MapState {
    pub fn new(map: &Rc<Map>) -> Self {
        let styles = vec![
            ('0', Style::default().fg(Color::Rgb(20, 20, 20))),
            ('1', Style::default().fg(Color::Blue)),
            ('2', Style::default().fg(Color::Yellow))
        ];
        Self {
            styles,
            scale: MIN_SCALE,
            shift_x: 0,
            shift_y: 0,
            map: Rc::clone(map),
        }
    }

    pub fn scroll_left(&mut self) {
        // TODO: check overflow
        self.shift_x += SHIFT_X;
    }

    pub fn scroll_right(&mut self) {
        // TODO: check overflow
        self.shift_x -= SHIFT_X;
    }

    pub fn scroll_up(&mut self) {
        // TODO: check overflow
        self.shift_y += SHIFT_Y;
    }

    pub fn scroll_down(&mut self) {
        // TODO: check overflow
        self.shift_y -= SHIFT_Y;
    }

    pub fn inc_scale(&mut self) {
        self.scale = min(self.scale + 1, MAX_SCALE);
    }

    pub fn dec_scale(&mut self) {
        self.scale = max(self.scale - 1, MIN_SCALE);
    }
}

pub struct MapWidget<'a> {
    block: Block<'a>,
}

impl<'a> MapWidget<'a> {
    pub fn new() -> Self {
        Self {
            block: Block::default().title("Map").borders(Borders::ALL),
        }
    }
}

impl<'a> StatefulWidget for MapWidget<'a> {
    type State = MapState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let inner = self.block.inner(area);
        self.block.render(area, buf);

        let mut render_hex = |pos_i: i32, pos_j: i32, style: &Style| {
            let start_x = state.shift_x
                + 3 * pos_j * state.scale;
            let start_y = state.shift_y
                + 2 * pos_i * state.scale + (pos_j % 2) * state.scale;

            let mut render_char = |x: i32, y: i32, ch: char, style: &Style| {
                let x = start_x + x;
                let y = start_y + y;
                if x < 0 || y < 0 {
                    return;
                }
                let x = x as u16;
                let y = y as u16;
                if x < inner.width && y < inner.height {
                    let cell = buf.get_mut(x + inner.x, y + inner.y);
                    cell.set_style(*style);
                    cell.set_char(ch);
                }
            };

            for i in 0..state.scale {
                // N
                render_char(state.scale + i, 0, '_', style);
                render_char(2 * state.scale + i, 0, '_', style);
                // NW
                render_char(i, state.scale - i, '╱', style);
                // NE
                render_char(3 * state.scale + i, i + 1, '╲', style);
                // SW
                render_char(i, state.scale + i + 1, '╲', style);
                // SE
                render_char(3 * state.scale + i, 2 * state.scale - i, '╱', style);
                // S
                render_char(state.scale + i, 2 * state.scale, '_', style);
                render_char(2 * state.scale + i, 2 * state.scale, '_', style);
            }
        };

        let grid = &(*state.map).grid;

        // TODO: rework grid traversal
        // Cell styles have priorities. Curent implementation traverses
        // whole grid for each cell style, so that cells with higher priority
        // styles will override ones with lower priority.
        // Complexity: num_of_cell_styles * grid_width * grid_height
        for (ch_type, style) in &state.styles {
            for (i, row) in grid.iter().enumerate() {
                for (j, ch) in row.chars().enumerate() {
                    if ch == *ch_type {
                        render_hex(i as i32, j as i32, style);
                    }
                }
            }
        }
    }
}
