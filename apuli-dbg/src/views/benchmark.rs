use super::util::{centered_rect, layout};
use crate::{App, AppState, Frame, Visibility};
use apuli_lib::{
    apuli::{query, rank},
    information::remaining_information,
};
use crossterm::event::KeyCode;
use ratatui::{layout::Layout, prelude::*, widgets::*};

const MAX_INDEX: usize = 4;

pub(crate) fn benchmarking_ui(frame: &mut Frame<'_>, app: &mut App) {
    let area = frame.size();

    // split area three way
    // STATISTICS | GAME SIMULATION VIEW | KEYMAP / ACTIONS
    let ar = area.right() / 3;
    let three_split = layout(area, Direction::Horizontal, vec![ar, ar, ar]);

    for i in 0..3 {
        let block = Block::new().on_black().title(format!("Window {i}"));
        frame.render_widget(block, three_split[i]);
    }
    word_list_view(frame, three_split[0], app);
    game_view(frame, three_split[1], app);
    action_view(frame, three_split[2], app);
}

fn game_view(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
    let AppState::BenchmarkView(visibility) = &app.state else {
        unreachable!();
    };
    if *visibility == Visibility::Hidden {
        return;
    }
    let rows = layout(area, Direction::Vertical, vec![3, 4, 4, 4, 4, 4, 4]);
    for row in 1..7 {
        let columns = layout(rows[row], Direction::Horizontal, vec![3, 6, 6, 6, 6, 6]);
        for col in 1..6 {
            let m = Margin::new(0, 0);
            let letter_box = columns[col].inner(&m);
            let s = Style::new().bg(Color::Rgb(80, 123, 58));
            let block = Block::new()
                .padding(Padding::new(1, 1, 0, 0))
                .borders(Borders::all())
                .style(s);
            let letter = Span::styled("A", Style::new().white().bold());
            frame.render_widget(block, letter_box);
            frame.render_widget(letter, letter_box.inner(&Margin::new(2, 1)));
        }
    }
}

fn action_view(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
    let block = Block::bordered().title("Choose action.").on_black();
    let popup_area = centered_rect(100, 60, area);

    let menu_items: [&str; MAX_INDEX] = [
        "Choose mode",
        "Run benchmarks",
        "Show/Hide games",
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

fn word_list_view(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
    let block = Block::bordered().title("Best words");
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let remaining_words = query(&[], None, None, app.word_lenght);
    let remaining_words: Vec<String> = rank(remaining_words)
        .into_iter()
        .map(|(score, word)| format!("{word} | {score}"))
        .collect();
    let remaining_count = remaining_words.len();
    let remaining_information = remaining_information(&remaining_words);
    let word_list = List::new(remaining_words).block(block);

    let info_block = Block::bordered().title("Info");
    let txt = Text::from(format!(
        "Words remaining: {remaining_count}\nInformation remaining:\n{remaining_information}"
    ));
    // frame.render_widget(Clear, area[0]);
    frame.render_stateful_widget(word_list, area[0], &mut app.menu_state);
    frame.render_widget(info_block, area[1]);
    frame.render_widget(txt, area[1].inner(&Margin::new(1, 1)));
}

pub(crate) fn benchmarking_input_listener(key: KeyCode, app: &mut App) {
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
                2 => {
                    let AS::BenchmarkView(current_visibility) = &app.state else {
                        unreachable!()
                    };
                    match current_visibility {
                        Visibility::Hidden => app.state = AS::BenchmarkView(Visibility::Shown),
                        Visibility::Shown => app.state = AS::BenchmarkView(Visibility::Hidden),
                    }
                }
                3 => app.state = AS::StatisticsView,
                _ => unreachable!("Ayo something is wrong"),
            }
            Some(current_sel)
        }
        KeyCode::Char('h') | KeyCode::Char('l') => {
            // toggle between action list and word list
            Some(0)
        }
        _ => Some(current_sel),
    };
    app.menu_state.select(new_index)
}
