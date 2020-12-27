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

const MAX_SCALE: u16 = 5;
const MIN_SCALE: u16 = 1;

pub struct MapState {
    styles: Vec<(char, Style)>,
    scale: u16,
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
            map: Rc::clone(map),
        }
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

        let mut render_hex = |pos_i: u16, pos_j: u16, style: &Style| {
            let start_x = 3 * pos_j * state.scale;
            let start_y = 2 * pos_i * state.scale + (pos_j % 2) * state.scale;

            let mut render_char = |x: u16, y: u16, ch: char, style: &Style| {
                let x = start_x + x;
                let y = start_y + y;
                if x < inner.width && y < inner.height {
                    let cell = buf.get_mut(x + inner.x, y + inner.y);
                    cell.set_style(*style);
                    cell.set_char(ch);
                    //buf.get_mut(x + inner.x, y + inner.y).set_char(ch);
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
                        render_hex(i as u16, j as u16, style);
                    }
                }
            }
        }
    }
}
