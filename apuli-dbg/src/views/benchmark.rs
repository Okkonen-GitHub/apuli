use ratatui::widgets::Clear;

use super::util::layout;
use crate::{App, AppState, Visibility};

pub type Frame<'a> = ratatui::Frame<'a>;
pub(crate) fn benchmarking_ui(frame: &mut Frame<'_>, app: &mut App) {
    let area = frame.size();
    frame.render_widget(Clear, area);
}
