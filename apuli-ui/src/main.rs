use yew::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Window};

mod components;
use crate::components::{manager::*, keyboard::Keyboard, board::Board, game::*, input::InputLoop};

use apuli_lib::apuli::ALLOWED_KEYS;

pub enum Msg {
    KeyPress(char),
    Enter,
    Backspace,
    ChangeWordLenght,
    UpdateTile(Tile),
    Clear,
}

struct App {
    keyboard_listener: Option<Closure<dyn Fn(KeyboardEvent)>>,
    input_handler: InputLoop,
    currect_game: Game,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            keyboard_listener: None,
            input_handler: InputLoop::new(5, Vec::new()),
            currect_game: Game::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        println!("Rendered");
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
                web_sys::console::log_1(&format!("{}", key).into());
                self.input_handler.insert_char(key);
                web_sys::console::log_1(&format!("{:?}", self.input_handler.current).into());
                self.currect_game.update_guesses(&self.input_handler);
            },
            Msg::Enter => {
                web_sys::console::log_1(&"Enter".into());
                if self.input_handler.current.len() == self.currect_game.word_length && self.currect_game.current_guess < 5 {
                    self.currect_game.current_guess += 1;
                    self.input_handler.current.clear() // who would want to insert the same word twice?
                } // TODO: go to the beginning maybe
                
            },
            Msg::Backspace => {
                web_sys::console::log_1(&"Backspace".into());
                self.input_handler.remove_char();
                self.currect_game.update_guesses(&self.input_handler);
            },
            Msg::ChangeWordLenght => {
                web_sys::console::log_1(&"Change word len".into());
            },
            Msg::UpdateTile(tile) => {
                web_sys::console::log_1(&format!("tile: {:?}", tile).into());
            },
            Msg::Clear => {
                println!("Clear"); // maybe just reload the page?
            },
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
            <div class={classes!("game", "dark")}>
                <div class="board-container">
                    <Board
                        guesses={self.currect_game.guesses.clone()} // clone for now..?
                        word_length={self.currect_game.word_length}
                        current_guess={self.currect_game.current_guess}
                    
                    
                        // guesses={vec![[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec()]}
                        // word_length={5}
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

fn main() {
    yew::start_app::<App>();
}
