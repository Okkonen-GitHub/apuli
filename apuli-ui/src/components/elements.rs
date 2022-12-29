use crate::Msg;
use yew::prelude::*;

use super::manager::TileManager;
use apuli_lib::apuli::query;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub callback: Callback<Msg>,
}

macro_rules! onmousedown {
    ( $cb:ident, $msg:expr ) => {{
        let $cb = $cb.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            $cb.emit($msg);
        })
    }};
}

#[function_component(ToggleButton)]
pub fn toggle_button(props: &ButtonProps) -> Html {
    let callback = props.callback.clone();

    html! {
        <>
        <div>
            {{
                let onmousedown = Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                callback.emit(Msg::ToggleAnswer);
                });
                html! {
                <button data-nosnippet="" class={classes!("btn", "correct")}
                                    onmousedown={onmousedown}>
                    { "VALMIS" }
                </button>
                }
            }}
        </div>
        </>

    }
}

#[function_component(ClearButton)]
pub fn clear_button(props: &ButtonProps) -> Html {
    let callback = props.callback.clone();

    html! {
        <>
        <div>
            {{
                let onmousedown = Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                callback.emit(Msg::Clear);
                });
                html! {
                <button data-nosnippet="" class={classes!("btn", "present")}
                                    onmousedown={onmousedown}>
                    { "RESET" }
                </button>
                }
            }}
        </div>
        </>

    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub on_toggle_answer_cb: Callback<MouseEvent>,
    pub on_toggle_help_cb: Callback<MouseEvent>,
    pub title: String,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_toggle_help_cb = props.on_toggle_help_cb.clone();
    let onclick_help = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_toggle_help_cb.emit(e);
        })
    };

    let on_toggle_menu_cb = props.on_toggle_answer_cb.clone();
    let onclick_menu = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_toggle_menu_cb.emit(e);
        })
    };

    html! {
    <header>
        <nav onclick={onclick_help} class="title-icon">{"?"}</nav>
                <h1 class="title">{&props.title}</h1>
            <nav onclick={onclick_menu} class="title-icon">{"≡"}</nav>
        </header>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct HelpModalProps {
    pub callback: Callback<Msg>,
}

#[function_component(HelpModal)]
pub fn help_modal(props: &HelpModalProps) -> Html {
    let callback = props.callback.clone();
    let toggle_help = onmousedown!(callback, Msg::ToggleHelp);

    html! {
        <div class="modal">
            <span onmousedown={toggle_help} class="modal-close">{"✖"}</span>
            <p><i>{"Sanuli "}</i>{"apu"}</p>
            <p>{"Syötä arvauksia ja muuta kirjainten värit vastaamaan omaa sanuli peliäsi ja kone kertoo kaikki mahdolliset vaihtoehdot, jotka ovat jäljellä"}</p>


        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AnswerModalProps {
    pub callback: Callback<Msg>,
    pub tile_manager: TileManager,
    pub word_length: usize,
}

#[function_component(AnswerModal)]
pub fn anser_modal(props: &AnswerModalProps) -> Html {
    let callback = props.callback.clone();
    let toggle_answer = onmousedown!(callback, Msg::ToggleAnswer);

    html! {
        <div class="modal">
            <span onmousedown={toggle_answer} class="modal-close">{"✖"}</span>
            {
            {
                let mngr = &mut props.tile_manager.clone();
                let oranges = mngr.gen_oranges();
                let blues = mngr.gen_blues(oranges.as_ref());
                let grays = mngr.gen_grays();

                let result = query(&grays, blues.as_ref(), oranges.as_ref(), props.word_length);


                result.iter().map(|word| {
                    html ! {
                        <p>{word}</p>
                    }
                }).collect::<Html>()
            }


            }

        </div>
    }
}
