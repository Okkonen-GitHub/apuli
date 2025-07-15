use super::util::centered_rect;
use crate::{views::benchmark::BenchmarkState, App, AppState, Frame};
use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};

const MAX_INDEX: usize = 5;

pub(crate) fn menu_ui(frame: &mut Frame<'_>, app: &mut App) {
    let area = frame.size();
    let block = Block::bordered().title("Choose action.").on_black();
    let popup_area = centered_rect(40, 40, area);

    let menu_items: [&str; MAX_INDEX] = [
        "Benchmark",
        &format!("Toggle word length ({})", app.word_lenght),
        "Filter",
        "Results",
        "Statistics",
    ];
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
    match key {
        KeyCode::Up => {
            let new_index = if current_sel > 0 {
                Some(current_sel - 1)
            } else {
                Some(MAX_INDEX - 1)
            };
            app.menu_state.select(new_index);
        }
        KeyCode::Down => {
            let new_index = if current_sel < MAX_INDEX - 1 {
                Some(current_sel + 1)
            } else {
                Some(0)
            };
            app.menu_state.select(new_index);
        }
        KeyCode::Enter => {
            use AppState as AS;
            match current_sel {
                0 => app.state = AS::BenchmarkView(BenchmarkState::default()),
                1 => {
                    if app.word_lenght == 5 {
                        app.word_lenght = 6;
                    } else {
                        app.word_lenght = 5;
                    }
                }
                2 => app.state = AS::FilterView,
                3 => app.state = AS::ResultView,
                4 => app.state = AS::StatisticsView,
                _ => unreachable!("Ayo something is wrong"),
            }
        }
        _ => (),
    };
}
