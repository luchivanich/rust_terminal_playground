use std::cmp::{max,min};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

const MAX_SCALE: u16 = 5;
const MIN_SCALE: u16 = 1;

pub struct MapState {
    scale: u16,
}

impl MapState {
    pub fn inc_scale(&mut self) {
        self.scale = min(self.scale + 1, MAX_SCALE);
    }

    pub fn dec_scale(&mut self) {
        self.scale = max(self.scale - 1, MIN_SCALE);
    }
}

impl Default for MapState {
    fn default() -> Self {
        Self {
            scale: MIN_SCALE,
        }
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
        // TODO: for test only, replace with real map
        render_hex(0, 0);
        render_hex(1, 0);
        render_hex(0, 2);
        render_hex(0, 1);

        render_hex(3, 0);
        render_hex(3, 1);
        render_hex(3, 2);
        render_hex(3, 3);
    }
}
