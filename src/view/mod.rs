mod map;

pub use map::MapState;

use map::MapWidget;
use tui::{
    backend::Backend,
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, map_state: &mut MapState) {
    let map_widget = MapWidget::new();
    f.render_stateful_widget(map_widget, f.size(), map_state);
}
