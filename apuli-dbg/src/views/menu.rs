use super::util::centered_rect;
use crate::{App, AppState};
use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};

pub type Frame<'a> = ratatui::Frame<'a>;

const MAX_INDEX: usize = 4;

pub(crate) fn menu_ui(frame: &mut Frame<'_>, app: &mut App) {
    let area = frame.size();
    let block = Block::bordered().title("Choose action.").bg(Color::Black);
    let popup_area = centered_rect(40, 40, area);

    let menu_items: [&str; MAX_INDEX] = ["Benchmark", "Filter", "Results", "Statistics"];
    // let mut state = ListState::default();
    let list = List::new(menu_items)
        .block(block)
        .highlight_style(
            Style::new().add_modifier(Modifier::REVERSED), // .fg(Color::LightBlue),
        )
        .highlight_symbol(" <> ")
        .repeat_highlight_symbol(true);

    frame.render_widget(Clear, popup_area);
    frame.render_stateful_widget(list, popup_area, &mut app.menu_state);
}

pub(crate) fn menu_input_listener(key: KeyCode, app: &mut App) {
    let current_sel = app.menu_state.selected().unwrap_or(0);
    let new_index = match key {
        KeyCode::Up => {
            if current_sel > 0 {
                Some(current_sel - 1)
            } else {
                Some(MAX_INDEX - 1)
            }
        }
        KeyCode::Down => {
            if current_sel < MAX_INDEX - 1 {
                Some(current_sel + 1)
            } else {
                Some(0)
            }
        }
        KeyCode::Enter => {
            use AppState as AS;
            match current_sel {
                0 => app.state = AS::BenchmarkView(crate::Visibility::Shown),
                1 => app.state = AS::FilterView,
                2 => app.state = AS::ResultView,
                3 => app.state = AS::StatisticsView,
                _ => unreachable!("Ayo something is wrong"),
            }
            Some(current_sel)
        }
        _ => Some(current_sel),
    };
    app.menu_state.select(new_index)
}
