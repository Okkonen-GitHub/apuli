use std::{iter, num::Wrapping};

use super::util::{centered_rect, layout};
use crate::{App, AppState, Frame, Visibility};
use apuli_lib::{
    apuli::{query, rank},
    information::remaining_information,
};
use crossterm::event::KeyCode;
use ratatui::{layout::Layout, prelude::*, style::Stylize, widgets::*};

const MAX_INDEX: usize = 4;

#[derive(Debug, Default, Clone, Copy)]
pub struct BenchmarkState {
    game_visible: Visibility,
    benchmarking_mode: BenchmarkingMode,
    selected_pane: BenchmarkPane,
    word_list_scroll: Wrapping<usize>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum BenchmarkPane {
    #[default]
    ActionMenu,
    Wordlist,
}

#[derive(Debug, Default, Clone, Copy)]
enum BenchmarkingMode {
    #[default]
    Everything,
    Single,
    CherryPick,
}

pub(crate) fn benchmarking_ui(frame: &mut Frame<'_>, bench: &mut BenchmarkState, app: &mut App) {
    let area = frame.size();

    // split area three way
    // STATISTICS | GAME SIMULATION VIEW | KEYMAP / ACTIONS
    let ar = area.right() / 3;
    let three_split = layout(area, Direction::Horizontal, vec![ar, ar, ar]);

    for i in 0..3 {
        let block = Block::new().on_black().title(format!("Window {i}"));
        frame.render_widget(block, three_split[i]);
    }
    word_list_view(frame, three_split[0], bench, app);
    game_view(frame, three_split[1], bench, app);
    action_view(frame, three_split[2], app);
}

fn game_view(frame: &mut Frame<'_>, area: Rect, bench: &mut BenchmarkState, _app: &mut App) {
    if bench.game_visible == Visibility::Hidden {
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
    // frame.render_widget(paragraph, area);
}

fn word_list_view(frame: &mut Frame<'_>, area: Rect, bench: &mut BenchmarkState, app: &mut App) {
    let block = Block::bordered().title(format!("Best words {}", bench.word_list_scroll.0));
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let remaining_words = query(&[], None, None, app.word_lenght);
    let word_list: Vec<_> = rank(remaining_words.clone()) // RC later
        .into_iter()
        .enumerate()
        .map(|(i, (score, word))| {
            Line::from(format!("{}. {word} | {score}", i + 1).add_modifier(
                if i == bench.word_list_scroll.0 {
                    Modifier::REVERSED
                } else {
                    Modifier::empty()
                },
            ))
        })
        .collect();
    let remaining_count = remaining_words.len();
    let remaining_information = remaining_information(&remaining_words);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scrollbar_state =
        ScrollbarState::new(remaining_count).position(bench.word_list_scroll.0);
    let word_list = Paragraph::new(word_list)
        .block(block)
        .scroll((bench.word_list_scroll.0 as u16, 0));

    let info_block = Block::bordered().title("Info");
    let txt = Text::from(format!(
        "Words remaining: {remaining_count}\nInformation remaining:\n{remaining_information}"
    ));
    // frame.render_widget(Clear, area[0]);
    frame.render_widget(word_list, area[0]);
    frame.render_stateful_widget(
        scrollbar,
        area[0].inner(&Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
    frame.render_widget(info_block, area[1]);
    frame.render_widget(txt, area[1].inner(&Margin::new(1, 1)));
}

pub(crate) fn benchmarking_input_listener(key: KeyCode, bench: &mut BenchmarkState, app: &mut App) {
    if bench.selected_pane == BenchmarkPane::Wordlist {
        match key {
            KeyCode::Up => bench.word_list_scroll -= 1,
            KeyCode::Down => bench.word_list_scroll += 1,
            _ => (),
        }
        app.state = AppState::BenchmarkView(*bench);
        return;
    }
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
                0 => app.state = AS::BenchmarkView(*bench),
                1 => app.state = AS::FilterView,
                2 => match bench.game_visible {
                    Visibility::Hidden => (*bench).game_visible = Visibility::Shown,
                    Visibility::Shown => {
                        (*bench).game_visible = Visibility::Hidden;
                        // dbg!(&bench);
                    }
                },
                3 => app.state = AS::StatisticsView,
                _ => unreachable!("Ayo something is wrong"),
            }
            app.state = AppState::BenchmarkView(*bench);
            return;
            // Some(current_sel)
        }
        KeyCode::Char('h') | KeyCode::Char('l') => {
            match bench.selected_pane {
                BenchmarkPane::Wordlist => bench.selected_pane = BenchmarkPane::ActionMenu,
                BenchmarkPane::ActionMenu => bench.selected_pane = BenchmarkPane::Wordlist,
            };
            Some(0)
        }
        _ => Some(current_sel),
    };
    app.menu_state.select(new_index);
    app.state = AppState::BenchmarkView(*bench);
}
