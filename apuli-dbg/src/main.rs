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
use views::menu::{menu_input_listener, menu_ui};

mod views;

#[derive(Debug)]
enum Visibility {
    Shown,
    Hidden,
}

#[derive(Default, Debug)]
enum AppState {
    #[default]
    MenuView,
    BenchmarkView(Visibility),
    FilterView,
    ResultView,
    StatisticsView,
}

#[derive(Default)]
pub(crate) struct App {
    state: AppState,
    menu_state: ListState,
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
    let result = query(&[], None, None, 5);
    let first = result.first().unwrap();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut dbg_app = App {
        menu_state: ListState::default().with_selected(Some(0)),
        ..Default::default()
    };

    loop {
        term.draw(|frame| {
            menu_ui(frame, &mut dbg_app);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
                use AppState as AS;
                match dbg_app.state {
                    AS::MenuView => menu_input_listener(key.code, &mut dbg_app),
                    _ => unimplemented!("MF"),
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
