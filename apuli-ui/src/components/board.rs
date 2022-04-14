use crate::components::game::Game;
use yew::prelude::*;

#[function_component(Board)]
pub fn board(props: &Game) -> Html {
    html! {
        <>
            <div class={classes!("board-6")}>
                {
                    props.guesses.iter().map(|guess|{
                        html! {
                            <div class={format!("row-{}", props.word_length)}>
                                {
                                    (0..props.word_length).map(|index| {
                                        let c = guess
                                        .get(index)
                                        .unwrap_or(&' ');

                                        html! {
                                            <div class="tile">
                                                { *c }
                                            </div>
                                        }
                                    }).collect::<Html>()

                                }
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}
