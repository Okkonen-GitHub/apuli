use std::fmt::Debug;

use components::elements::AnswerModal;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Window};
use yew::prelude::*;

mod components;
use crate::components::{
    board::Board,
    elements::{ClearButton, Header, HelpModal, MenuModal, ToggleButton},
    game::*,
    input::InputLoop,
    keyboard::Keyboard,
    manager::*,
};

use apuli_lib::apuli::ALLOWED_KEYS;

pub enum Msg {
    KeyPress(char),
    Enter,
    Backspace,
    ChangeWordLength(usize),
    UpdateTile(Tile),
    Clear,
    ToggleAnswer,
    ToggleHelp,
    ToggleMenu,
    ChangeTheme(Theme),
}

struct App {
    keyboard_listener: Option<Closure<dyn Fn(KeyboardEvent)>>,
    input_handler: InputLoop,
    currect_game: Game,
    tile_manager: TileManager,
    is_help_visible: bool,
    is_answer_visible: bool,
    is_menu_visible: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            keyboard_listener: None,
            input_handler: InputLoop::new(5, Vec::new()),
            currect_game: Game::new(5, Theme::Dark),
            tile_manager: TileManager::new(),
            is_help_visible: false,
            is_answer_visible: false,
            is_menu_visible: false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let window: Window = window().expect("window not available");

        let cb = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key().chars().count() == 1 {
                let key = e.key().to_uppercase().chars().next().unwrap();
                if ALLOWED_KEYS.contains(&key) && !e.ctrl_key() && !e.alt_key() && !e.meta_key() {
                    e.prevent_default();
                    Some(Msg::KeyPress(key))
                } else {
                    None
                }
            } else if e.key() == "Backspace" {
                e.prevent_default();
                Some(Msg::Backspace)
            } else if e.key() == "Enter" {
                e.prevent_default();
                Some(Msg::Enter)
            } else {
                None
            }
        });

        let listener =
            Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |e: KeyboardEvent| cb.emit(e)));

        window
            .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
            .unwrap();
        self.keyboard_listener = Some(listener);
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyPress(key) => {
                self.input_handler.insert_char(key);
                //web_sys::console::log_1(&format!("{:?}", self.input_handler.current).into());
                self.currect_game.update_guesses(&self.input_handler);
            }
            Msg::Enter => {
                if self.input_handler.current.len() == self.currect_game.word_length
                    && self.currect_game.current_guess < 5
                {
                    self.currect_game.current_guess += 1;
                    self.input_handler.current = self
                        .currect_game
                        .guesses
                        .get(self.currect_game.current_guess)
                        .unwrap()
                        .to_vec();
                } else if self.currect_game.current_guess == 5
                    && self.input_handler.current.len() == self.currect_game.word_length
                {
                    self.currect_game.current_guess = 0;
                    self.input_handler.current = self.currect_game.guesses.get(0).unwrap().to_vec();
                }
            }
            Msg::Backspace => {
                self.input_handler.remove_char();
                self.currect_game.update_guesses(&self.input_handler);
            }
            Msg::ChangeWordLength(word_length) => {
                if word_length == self.currect_game.word_length {
                    self.is_menu_visible = false;
                    return true;
                }
                self.input_handler.word_len = word_length; //we don't want it to remember old stuff
                self.input_handler.current.clear(); // so it automatically clears all the state
                self.tile_manager.tiles.clear();
                self.currect_game = Game::new(word_length, self.currect_game.theme);
                self.is_menu_visible = false;
            }
            Msg::UpdateTile(tile) => self.tile_manager.update_tile(tile),
            Msg::Clear => {
                println!("Clear"); // maybe just reload the page?
                self.currect_game = Game::new(self.currect_game.word_length, self.currect_game.theme); // I guess replacing the game state with the
                                                                              // default game state works?
                self.input_handler.current.clear(); // gotta remember to clear the input loop
                self.tile_manager.tiles.clear(); // also gotta remember to clear tilestates
            }
            Msg::ToggleHelp => {
                self.is_help_visible = !self.is_help_visible;
                self.is_answer_visible = false;
                self.is_menu_visible = false;
            }
            Msg::ToggleAnswer => {
                self.is_answer_visible = !self.is_answer_visible;
                self.is_help_visible = false;
                self.is_menu_visible = false;
            }
            Msg::ToggleMenu => {
                self.is_menu_visible = !self.is_menu_visible;
                self.is_answer_visible = false;
                self.is_help_visible = false;
            }
            Msg::ChangeTheme(theme) => {
                if self.currect_game.theme != theme {
                self.currect_game.theme = theme;
            }
                self.is_menu_visible = false;
            }
        }
        true
    }

    fn destroy(&mut self, _: &Context<Self>) {
        // Remove the keyboard listener
        if let Some(listener) = self.keyboard_listener.take() {
            let window: Window = window().expect("window not available");
            window
                .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                .unwrap();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let keyboard_state: Vec<char> = ALLOWED_KEYS.iter().map(|c| *c).collect();
        // let guesses = self.currect_game.guesses ;
        html! {
            <div class={classes!("game", self.currect_game.theme.to_string())}>
                <Header
                    on_toggle_help_cb={link.callback(|_| Msg::ToggleHelp)}
                    on_toggle_answer_cb={link.callback(|_| Msg::ToggleMenu)}
                    title={"Apuli"}
                />
                <div class="board-container">
                    <Board
                        guesses={self.currect_game.guesses.clone()} // clone for now..?
                        current_guess={self.currect_game.current_guess}
                        word_length={self.currect_game.word_length}
                        cb={link.callback(move |msg| msg)}
                        tile_states={self.tile_manager.clone()}
                    />
                </div>
                {
                    if self.is_help_visible {
                        html! { <HelpModal callback={link.callback(move |msg| msg)} /> }
                    } else if self.is_answer_visible {
                        html! {
                            <AnswerModal
                                callback={link.callback(move |msg| msg) }
                                tile_manager={self.tile_manager.clone()}
                                word_length={self.currect_game.word_length}

                            />
                        }
                    }
                    else if self.is_menu_visible {
                        html! {
                            <MenuModal
                                callback={link.callback(move |msg| msg)}
                                word_length={self.currect_game.word_length}
                                theme={self.currect_game.theme}
                            />
                        }
                    } else {
                        html! {}
                    }
                }
                <div class="btn-container">
                    <ToggleButton
                        callback={link.callback(move |msg| msg)}
                    />
                    <ClearButton
                        callback={link.callback(move |msg| msg)}
                    />
                </div>
                <Keyboard
                    callback={link.callback(move |msg| msg)}
                    message={"hellou".to_string()}
                    word={"hello".to_string()}
                    keyboard={keyboard_state}
                />
            </div>
        }
    }
}

pub fn cprint(m: impl Debug) -> () {
    web_sys::console::log_1(&format!("{:#?}", m).into());
}

fn main() {
    yew::start_app::<App>();
}
