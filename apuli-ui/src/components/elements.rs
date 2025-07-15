use std::{fmt::Display, iter::Map};

use crate::Msg;
use apuli_lib::apuli::rank_scout;
use yew::prelude::*;

use super::{
    game::{GameMode, Theme},
    manager::TileManager,
};
use apuli_lib::{
    apuli::{query, rank, rank_combined, Letter},
    information::rank_entropy,
};

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
    let onmousedown = onmousedown!(callback, Msg::ToggleAnswer);

    html! {
        <>
            <div>
                <button data-nosnippet="" class={classes!("btn", "correct")}
                                    onmousedown={onmousedown}>
                    { "VALMIS" }
                </button>
            </div>
        </>

    }
}

#[function_component(ClearButton)]
pub fn clear_button(props: &ButtonProps) -> Html {
    let callback = props.callback.clone();
    let onmousedown = onmousedown!(callback, Msg::Clear);

    html! {
        <>
            <div>
                <button data-nosnippet="" class={classes!("btn", "present")}
                                    onmousedown={onmousedown}>
                    { "RESET" }
                </button>
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
            <p>{"Apuli - Sanuli auttaja"}</p>
            <p>{"Syötä arvauksia ja muuta kirjainten värit vastaamaan omaa sanuli peliäsi klikkailemallla kirjaimia. Kun olet syöttänyt arvaukset, valittu algoritmi kertoo optimaalin seuraavan arvauksen."}</p>
            <p>
                {"Algoritmit:"}
            </p>
            <p>
                {"Perus: Yksinkertainen, mutta nopea."}
            </p>
            <p>
                {"Informaatio: Hitaampi, täysin matemaattinen algoritmi. Laskee kunkin arvauksen tuottaman informaation oletusarvon."}
            </p>
            <p>
                {"Yhdistetty: Hitaampi algoritmi, joka etsii kaikkien sanojen joukosta ne, jotka sisältävät eniten yhteisiä kirjaimia."}
            </p>
            <p>
                {"Sanalistojen pohjana on käytetty Kotimaisten kielten keskuksen (Kotus) julkaiseman "}
                <a href="https://creativecommons.org/licenses/by/3.0/deed.fi" class="link">
                    { "CC Nimeä 3.0 Muokkaamaton" }
                </a>
                {"-lisensoidun nykysuomen sanalistan sanoja, joista on karsittu ja lisätty tarpeen mukaan."}
            </p>
            <div class="version">
                <a class="version" href={"https://github.com/okkonen-github/apuli"} target="_blank">{"Apuli-dev | lähteet"}</a>
            </div>
        </div>
    }
}

// T would be u16 if I hadn't made different ranking methods use different types
// This is because theoretically it is possible to have a word with negative score in some cases
// Shows first n ... last m
fn show_n_answers<T>(words: Vec<(T, String)>, n: usize, m: usize) -> Html
where
    T: Display,
{
    // a little overcomplicated maybe?
    fn to_html<'a, T: 'a + Display, I: Iterator<Item = (usize, &'a (T, String))>>(
        iter: I,
    ) -> Map<I, impl FnMut((usize, &'a (T, String))) -> yew::virtual_dom::VNode> {
        iter.map(|(index, (score, word))| {
            html! {
                <p class="answer">
                    // Should say "Bits" if score is in bits
                    {format!("{index}.  {word}")} <wbr/> {format!("  (VR:{score:.3})") }
                </p>
            }
        })
    }
    let dots = std::iter::once("...").map(|c| {
        html! {
            <p>{c}</p>
        }
    });
    let first = words.iter().take(n).enumerate();
    let first = to_html(first);
    let last = words
        .iter()
        .enumerate()
        .skip(n) // remove duplicates
        .rev()
        .take(m)
        .rev();
    let last = to_html(last);
    if last.len() > 0 {
        first.chain(dots).chain(last).collect::<Html>()
    } else {
        first.collect::<Html>()
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum AnswerType {
    Basic,
    Information,
    Scout,
}

#[derive(Properties, PartialEq)]
pub struct AnswerModalProps {
    pub callback: Callback<Msg>,
    pub tile_manager: Vec<TileManager>,
    pub word_length: usize,
    pub game_mode: GameMode,
    pub answer_mode: AnswerType,
}

#[function_component(AnswerModal)]
pub fn answer_modal(props: &AnswerModalProps) -> Html {
    let callback = props.callback.clone();
    let toggle_answer = onmousedown!(callback, Msg::ToggleAnswer);

    let set_basic = onmousedown!(callback, Msg::ChangeAnswerMode(AnswerType::Basic));
    let set_information = onmousedown!(callback, Msg::ChangeAnswerMode(AnswerType::Information));
    let set_scout = onmousedown!(callback, Msg::ChangeAnswerMode(AnswerType::Scout));

    let mngr = props.tile_manager.clone();

    html! {
    <>
        <div class="modal">
            <span onmousedown={toggle_answer} class="modal-close answer-modal">
                {"✖"}
            </span>
            <div>
                <label class="label">{"Valitse algoritmi"}</label>
                <div class="select-container">
                    <button class={classes!("select", (props.answer_mode == AnswerType::Basic).then_some("select-active"))}
                        onmousedown={set_basic}>
                        { "Perus" }
                    </button>
                    <button class={classes!("select", (props.answer_mode == AnswerType::Information).then_some("select-active"))}
                        onmousedown={set_information}>
                        { "Informaatio" }
                    </button>
                    <button class={classes!("select", (props.answer_mode == AnswerType::Scout).then_some("select-active"))}
                        onmousedown={set_scout}>
                        { "Yhdistetty" }
                    </button>

                </div>
            </div>
            {
                if props.game_mode == GameMode::Neluli {
                    html! {
                        <>


                        {{
                            match props.answer_mode {
                                AnswerType::Basic => {
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
                                                                            show_n_answers(ranked, 25, 3)
                                                                        }}
                                                                }
                                                            }
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    }
                                },
                                AnswerType::Information => {
                                    // refactor this maybe
                                    html! {
                                        <div class="neluli-answer">
                                            {
                                                (0..4).into_iter().map(|i| {
                                                    html! {
                                                        <div class="answer-container">
                                                            {
                                                                html! {
                                                                        {{
                                                                            let mngr = &mut mngr[i].clone();
                                                                            let oranges = mngr.gen_oranges();
                                                                            let blues = mngr.gen_blues(/*oranges.as_ref()*/);
                                                                            let grays = mngr.gen_grays();
                                                                            let result = query(&grays, blues, oranges, props.word_length);
                                                                            let ranked = rank_entropy(&result);
                                                                            let remaining_entropy = (1000.0*(ranked.len() as f64).log2()).round() / 1000.0;
                                                                            html! {
                                                                                <>
                                                                                <p>
                                                                                    { format!("Jäljellä: {remaining_entropy:.3}") }
                                                                                </p>
                                                                                { show_n_answers(ranked, 25, 3) }
                                                                                </>
                                                                            }
                                                                        }}
                                                                }
                                                            }
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    }
                                },
                                AnswerType::Scout => {
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
                                    show_n_answers(ranked, 25, 3)
                                }
                            }
                        }}
                    </>
                    }
                } else { // props.game_mode == GameMode::Sanuli
                    html! {
                        <>


                            {{
                                let mngr = &mut mngr[0].clone();
                                let oranges = mngr.gen_oranges();
                                let blues = mngr.gen_blues(/*oranges.as_ref()*/);
                                let grays = mngr.gen_grays();
                                let result = query(&grays, blues, oranges, props.word_length);
                                match props.answer_mode {
                                    AnswerType::Basic => {
                                        let ranked = rank(result);
                                        show_n_answers(ranked, 25, 3)
                                    },
                                    AnswerType::Scout => {
                                        let ranked = rank_scout(result, props.word_length);
                                        show_n_answers(ranked, 25, 3)
                                    }
                                    AnswerType::Information => {
                                        let ranked = rank_entropy(&result);
                                        let remaining_entropy = (ranked.len() as f64).log2();
                                        html! {
                                            <>
                                            <p>
                                                { format!("Jäljellä: {remaining_entropy:.3}") }
                                            </p>
                                            { show_n_answers(ranked, 25, 3) }
                                            </>
                                        }
                                    }
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
