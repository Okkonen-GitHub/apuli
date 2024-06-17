use crate::{App, AppState};

pub type Frame<'a> = ratatui::Frame<'a>;
pub(crate) fn benchmarking_ui(frame: &mut Frame<'_>, app: &mut App) {}
