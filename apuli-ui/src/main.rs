use yew::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Window};

mod components;
use crate::components::{manager::*, keyboard::Keyboard, board::Board, game::Game};

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
    keyboard_listener: Option<Closure<dyn Fn(KeyboardEvent)>>
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            keyboard_listener: None,
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
            },
            Msg::Enter => {
                web_sys::console::log_1(&"Enter".into());
            },
            Msg::Backspace => {
                web_sys::console::log_1(&"Backspace".into());
            },
            Msg::ChangeWordLenght => {
                web_sys::console::log_1(&"Change word len".into());
                println!("ChangeWordLenght");
            },
            Msg::UpdateTile(tile) => {
                web_sys::console::log_1(&format!("tile: {:?}", tile).into());
            },
            Msg::Clear => {
                println!("Clear");
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
               
        html! {
            <div class={classes!("game", "dark")}>
                <div class="board-container">
                    <Board
                        guesses={vec![[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec()]}
                        word_length={5}
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