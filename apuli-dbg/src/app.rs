use ratatui::{prelude::*, widgets::*};

pub type Frame<'a> = ratatui::Frame<'a>;

pub(crate) fn menu_ui(frame: &mut Frame<'_>) {
    let area = frame.size();
    frame.render_widget(Paragraph::new("Generate results").white().on_black(), area);
}
