use yew::prelude::*;

// use crate::components::manager::TileState;
use crate::Msg;


const KEYBOARD_0: [char; 10] = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'];
const KEYBOARD_1: [char; 11] = ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Ö', 'Ä'];
const KEYBOARD_2: [char; 7] = ['Z', 'X', 'C', 'V', 'B', 'N', 'M'];

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<Msg>,


    pub message: String,
    pub word: String,

    pub keyboard: Vec<char>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &Props) -> Html {
    let callback = props.callback.clone();
    let onbackspace = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        callback.emit(Msg::Backspace);
    });

    html! {
        <div class="keyboard">
            // {
            //     html! {
            //         <Message
            //             message={props.message.clone()}
            //             word={props.word.clone()}
            //             callback={props.callback.clone()}
            //         />
            //     }
            // }

            <div class="keyboard-row">
                {
                    KEYBOARD_0.iter().map(|key| {
                        let callback = props.callback.clone();
                        let onkeypress = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();

                            callback.emit(Msg::KeyPress(*key));
                        });

                        html! {
                            <KeyboardButton character={*key} onkeypress={onkeypress} />
                        }
                    }).collect::<Html>()
                }
                <button data-nosnippet="" class={classes!("keyboard-button", "keyboard-button-backspace")} onmousedown={onbackspace}>
                    { "⌫" }
                </button>
            </div>
            <div class="keyboard-row">
                <div class="spacer" />
                {
                    KEYBOARD_1.iter().map(|key| {
                        let callback = props.callback.clone();
                        let onkeypress = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            callback.emit(Msg::KeyPress(*key));
                        });

                        html! {
                            <KeyboardButton character={*key}  onkeypress={onkeypress}/>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="keyboard-row">
                <div class="spacer" />
                <div class="spacer" />
                <div class="spacer" />
                {
                    KEYBOARD_2.iter().map(|key| {
                        let callback = props.callback.clone();
                        let onkeypress = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            callback.emit(Msg::KeyPress(*key));
                        });

                        html! {
                            <KeyboardButton character={*key}  onkeypress={onkeypress}/>
                        }
                    }).collect::<Html>()
                }
                {
                    //TODO FIX
                    // if props.is_guessing {
                    if true {
                        let callback = props.callback.clone();
                        let onmousedown = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            callback.emit(Msg::Enter);
                        });

                        html! {
                            <button data-nosnippet="" class={classes!("keyboard-button", "keyboard-button-submit")}
                                onmousedown={onmousedown}>
                                { "ARVAA" }
                            </button>
                        }
                    } else {
                        let callback = props.callback.clone();
                        let onmousedown = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            callback.emit(Msg::Clear);
                        });

                        html! {
                            <button data-nosnippet="" class={classes!("keyboard-button", "keyboard-button-submit", "correct")}
                                onmousedown={onmousedown}>
                                { "UUSI?" }
                            </button>
                        }
                    }
                }
                <div class="spacer" />
                <div class="spacer" />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyboardButtonProps {
    pub onkeypress: Callback<MouseEvent>,
    pub character: char,
}

#[function_component(KeyboardButton)]
pub fn keyboard_button(props: &KeyboardButtonProps) -> Html {
    html! {
        <button data-nosnippet="" class={classes!("keyboard-button", "unknown")}
            onmousedown={props.onkeypress.clone()}>
            { props.character }
        </button>
    }
}
