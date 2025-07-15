use std::fmt::Display;

use crate::Msg;
use apuli_lib::apuli::rank_scout;
use yew::prelude::*;

use super::{
    game::{GameMode, Theme},
    manager::TileManager,
};
use apuli_lib::apuli::{query, rank, rank_combined, Letter};

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
            <p>{"Sanuli apu"}</p>
            <p>{"Syötä arvauksia ja muuta kirjainten värit vastaamaan omaa sanuli peliäsi ja kone kertoo kaikki mahdolliset vaihtoehdot, jotka ovat jäljellä"}</p>
            <p href="https://creativecommons.org/licenses/by/3.0/deed.fi">
                {"Sanalistojen pohjana on käytetty Kotimaisten kielten keskuksen (Kotus) julkaiseman "}
                <a href="https://creativecommons.org/licenses/by/3.0/deed.fi" class="link">
                    { "CC Nimeä 3.0 Muokkaamaton" }
                </a>
                {"-lisensoidun nykysuomen sanalistan sanoja, joista on karsittu ja lisätty tarpeen."}
            </p>

        </div>
    }
}

// T would be u16 if I hadn't made different ranking methods use different types
// This is because theoretically it is possible to have a word with negative score in some cases
fn show_n_answers<T>(words: Vec<(T, String)>, n: usize) -> Html
where
    T: Display,
{
    let words = words.iter().take(n).enumerate();
    words
        .map(|(index, (score, word))| {
            html! {
                <p class="answer">
                    {index}{".  "}{word} <wbr/> {format!("  (VR:{score})") }
                </p>
            }
        })
        .collect::<Html>()
}

#[derive(Properties, PartialEq)]
pub struct AnswerModalProps {
    pub callback: Callback<Msg>,
    pub tile_manager: Vec<TileManager>,
    pub word_length: usize,
    pub game_mode: GameMode,
    pub show_combined: bool,
}

#[function_component(AnswerModal)]
pub fn answer_modal(props: &AnswerModalProps) -> Html {
    let callback = props.callback.clone();
    let toggle_answer = onmousedown!(callback, Msg::ToggleAnswer);

    let toggle_combined = onmousedown!(callback, Msg::ToggleCombined);

    let mngr = props.tile_manager.clone();

    html! {
    <>
        <div class="modal">
            <span onmousedown={toggle_answer} class="modal-close answer-modal">
                {"✖"}
            </span>
            {
                if props.game_mode == GameMode::Neluli {
                    html! {
                        <>
                            <div>
                                <label class="label">{"Yhdistetty?"}</label>
                                <div class="select-container">
                                    <button class={classes!("select", (props.show_combined).then_some("select-active"))}
                                        onmousedown={toggle_combined}>
                                        { if props.show_combined {"Yhdistetty"} else {"Erikseen"} }
                                    </button>
                                </div>
                            </div>

                        {{
                            if !props.show_combined {
                                html! {
                                    <div class="neluli-answer">
                                        {
                                            (0..4).into_iter().map(|i| {
                                                html! {
                                                    <div class="answer-container">
                                                        {
                                                            html ! {
                                                                    {{
                                                                        let mngr = &mut mngr[i].clone();
                                                                        let oranges = mngr.gen_oranges();
                                                                        let blues = mngr.gen_blues(/*oranges.as_ref()*/);
                                                                        let grays = mngr.gen_grays();
                                                                        let result = query(&grays, blues, oranges, props.word_length);
                                                                        let ranked = rank(result);
                                                                        show_n_answers(ranked, 25)
                                                                    }}
                                                            }
                                                        }
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                    }
                            } else {

                                let mngr = &mut mngr.clone();

                                let mut oranges: Vec<Option<Vec<Letter>>> = Vec::new();
                                let mut blues: Vec<Option<Vec<Letter>>> = Vec::new();
                                let mut grays: Vec<Vec<Letter>> = Vec::new();
                                for board_mngr in mngr.iter().take(4) {
                                    let mut manager = board_mngr.clone();
                                    oranges.push(manager.gen_oranges());
                                    blues.push(manager.gen_blues());
                                    grays.push(manager.gen_grays());
                                }
                                let mut words = Vec::new();
                                // let blues_n = blues.clone();
                                for i in 0..4 {
                                    // Should check for duplicates BUT increase score for the
                                    // duplicates
                                    // cprint(&grays[i]); cprint(&i);
                                    let res = query(&grays[i], blues[i].clone(), oranges[i].clone(), props.word_length);
                                    if res.len() != 1 {
                                        res.iter().for_each(|word| {
                                            words.push(word.clone());
                                        });
                                    }

                                }
                                // cprint(&words);
                                let ranked = rank_combined(&grays, blues, &oranges, words);
                                show_n_answers(ranked, 25)

                            }
                        }}
                    </>
                    }
                } else { // props.game_mode == GameMode::Sanuli
                    html! {
                        <>
                            <div>
                                <label class="label">{"Tiedustelu tila"}</label>
                                <div class="select-container">
                                    <button class={classes!("select", (props.show_combined).then_some("select-active"))}
                                            onmousedown={toggle_combined}>
                                            {if props.show_combined {"Tiedustelu"} else {"Tavallinen"}}
                                        </button>
                                    </div>
                            </div>

                            {{
                                let mngr = &mut mngr[0].clone();
                                let oranges = mngr.gen_oranges();
                                let blues = mngr.gen_blues(/*oranges.as_ref()*/);
                                let grays = mngr.gen_grays();
                                let result = query(&grays, blues, oranges, props.word_length);
                                if !props.show_combined {
                                    let ranked = rank(result);

                                    show_n_answers(ranked, 25)

                                } else {
                                    let ranked = rank_scout(result, props.word_length);
                                    show_n_answers(ranked, 25)
                                }
                            }}
                        </>
                    }
                }
            }
        </div>
    </>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct MenuModalProps {
    pub callback: Callback<Msg>,
    pub word_length: usize,
    pub theme: Theme,
    pub mode: GameMode,
}

#[function_component(MenuModal)]
pub fn menu_modal(props: &MenuModalProps) -> Html {
    let callback = props.callback.clone();
    let toggle_menu = onmousedown!(callback, Msg::ToggleMenu);

    let change_word_length_5 = onmousedown!(callback, Msg::ChangeWordLength(5));
    let change_word_length_6 = onmousedown!(callback, Msg::ChangeWordLength(6));

    let change_theme_colorblind = onmousedown!(callback, Msg::ChangeTheme(Theme::Colorblind));
    let change_theme_dark = onmousedown!(callback, Msg::ChangeTheme(Theme::Dark));

    let change_mode_sanuli = onmousedown!(callback, Msg::ChangeMode(GameMode::Sanuli));
    let change_mode_neluli = onmousedown!(callback, Msg::ChangeMode(GameMode::Neluli));

    html! {
        <div class="modal">
            <span onmousedown={toggle_menu} class="modal-close">{"✖"}</span>
                <div>
                    <label class="label">{"Sanulien pituus:"}</label>
                    <div class="select-container">
                        <button class={classes!("select", (props.word_length == 5).then_some("select-active"))}
                                onmousedown={change_word_length_5}>
                            {"5 merkkiä"}
                            </button>
                        <button class={classes!("select", (props.word_length == 6).then_some("select-active"))}
                                onmousedown={change_word_length_6}>
                            {"6 merkkiä"}
                        </button>
                    </div>
                </div>
                <div>
                    <label class="label">{"Pelimuoto:"}</label>
                    <div class="select-container">
                        <button class={classes!("select", (props.mode == GameMode::Sanuli).then_some("select-active"))}
                                onmousedown={change_mode_sanuli}>
                            {"Sanuli"}
                            </button>
                        <button class={classes!("select", (props.mode == GameMode::Neluli).then_some("select-active"))}
                                onmousedown={change_mode_neluli}>
                            {"Neluli"}
                        </button>
                    </div>
                </div>
                <div>
                    <label class="label">{"Teema:"}</label>
                    <div class="select-container">
                        <button class={classes!("select", (props.theme == Theme::Dark).then_some("select-active"))}
                                onmousedown={change_theme_dark}>
                            {"Tumma"}
                            </button>
                        <button class={classes!("select", (props.theme == Theme::Colorblind).then_some("select-active"))}
                                onmousedown={change_theme_colorblind}>
                            {"Värisokeille"}
                        </button>
                    </div>
                </div>

            <div class="version">
                <a class="version" href={"https://github.com/okkonen-github/apuli"} target="_blank">{"Apuli-dev | lähteet"}</a>
            </div>
        </div>
    }
}
