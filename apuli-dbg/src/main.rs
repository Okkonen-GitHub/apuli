use apuli_lib::apuli::query;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::{ListState, Paragraph},
};
use std::{
    io::{stdout, Result},
    panic,
};
use views::{
    benchmark::BenchmarkState,
    menu::{menu_input_listener, menu_ui},
};

use crate::views::benchmark::{benchmarking_input_listener, benchmarking_ui};

mod bench;
mod games;
mod views;

pub type Frame<'a> = ratatui::Frame<'a>;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
enum Visibility {
    #[default]
    Shown,
    Hidden,
}

#[derive(Default, Debug)]
enum AppState {
    #[default]
    MenuView,
    BenchmarkView(BenchmarkState),
    FilterView,
    ResultView,
    StatisticsView,
}

#[derive(Default)]
pub(crate) struct App {
    state: AppState,
    menu_state: ListState,
    word_lenght: usize,
}

fn panic_handler() {
    let og_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        og_hook(info);
    }))
}

fn main() -> Result<()> {
    panic_handler();
    use AppState as AS;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut dbg_app = App {
        menu_state: ListState::default().with_selected(Some(0)),
        word_lenght: 5,
        ..Default::default()
    };

    loop {
        term.draw(|frame| match dbg_app.state {
            AS::MenuView => menu_ui(frame, &mut dbg_app),
            AS::BenchmarkView(ref mut bench) => {
                benchmarking_ui(frame, &mut bench.clone(), &mut dbg_app)
            }
            _ => unimplemented!("It's not ready"),
        })?;

        if event::poll(std::time::Duration::from_millis(20))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        // main keybind handler
                        KeyCode::Char('q') => break,
                        KeyCode::Char('m') => {
                            dbg_app.state = AS::MenuView;
                            dbg_app.menu_state.select(Some(0));
                        }
                        _ => (),
                    }
                    match dbg_app.state {
                        AS::MenuView => menu_input_listener(key.code, &mut dbg_app),
                        AS::BenchmarkView(ref mut bench) => {
                            benchmarking_input_listener(key.code, &mut bench.clone(), &mut dbg_app)
                        }
                        _ => unimplemented!("MF"),
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
