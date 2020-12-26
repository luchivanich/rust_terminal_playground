use crate::model::Map;
use std::{
    cmp::{max,min},
    rc::Rc,
};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

const MAX_SCALE: u16 = 5;
const MIN_SCALE: u16 = 1;

pub struct MapState {
    scale: u16,
    map: Rc<Map>,
}

impl MapState {
    pub fn new(map: &Rc<Map>) -> Self {
        Self {
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

        let mut render_hex = |pos_i: u16, pos_j: u16| {
            let start_x = 3 * pos_j * state.scale;
            let start_y = 2 * pos_i * state.scale + (pos_j % 2) * state.scale;

            let mut render_char = |x: u16, y: u16, ch: char| {
                let x = start_x + x;
                let y = start_y + y;
                if x < inner.width && y < inner.height {
                    buf.get_mut(x + inner.x, y + inner.y).set_char(ch);
                }
            };

            for i in 0..state.scale {
                // N
                render_char(state.scale + i, 0, '_');
                render_char(2 * state.scale + i, 0, '_');
                // NW
                render_char(i, state.scale - i, '╱');
                // NE
                render_char(3 * state.scale + i, i + 1, '╲');
                // SW
                render_char(i, state.scale + i + 1, '╲');
                // SE
                render_char(3 * state.scale + i, 2 * state.scale - i, '╱');
                // S
                render_char(state.scale + i, 2 * state.scale, '_');
                render_char(2 * state.scale + i, 2 * state.scale, '_');
            }
        };

        let grid = &(*state.map).grid;

        for (i, row) in grid.iter().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                if ch != ' ' {
                    render_hex(i as u16, j as u16);
                }
            }
        }
    }
}
