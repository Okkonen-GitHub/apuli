use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;
pub type Frame<'a> = ratatui::Frame<'a>;

pub(crate) fn menu_ui(frame: &mut Frame<'_>) {
    let area = layout(frame.size(), Direction::Vertical, vec![1, 1, 1]);
    frame.render_widget(
        Paragraph::new("Generate results").white().on_black(),
        area[0],
    );
    frame.render_widget(Paragraph::new("Filter results").white().on_black(), area[1]);
    frame.render_widget(Paragraph::new("Show results").white().on_black(), area[2]);
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}
