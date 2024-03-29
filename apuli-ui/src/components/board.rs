use super::{game::GameMode, manager::*};
use crate::Msg;
use yew::prelude::*;

use super::manager::TileManager;
//use crate::components::manager::*;
//use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub guesses: Vec<Vec<char>>,
    pub word_length: usize,
    pub cb: Callback<Msg>,
    pub tile_states: TileManager,
    pub current_guess: usize,
    pub mode: GameMode,
    pub max_guesses: usize,
    pub board_index: usize,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let tiles = &props.tile_states.tiles;
    let board_index = props.board_index;

    //let guesses = props.clone().guesses;
    html! {
        <>
            <div class={classes!(format!("board-{}", props.max_guesses))}>
                {
                    props.guesses.iter().map(|guess| {
                        html! {
                            <div class={format!("row-{}", props.word_length)}>
                                {
                                    (0..props.word_length).map(|index| {
                                        let c = guess
                                            .get(index)
                                            .unwrap_or(&' ');

                                        let character = *c;
                                        let callback = props.cb.clone();
                                        let mut state: String = "".into();

                                        let mut new_state = TileState::Black;
                                        // cprint(tiles);
                                        for tile in tiles {
                                            if tile.character == *c && tile.position == index {
                                                match tile.state {
                                                    TileState::Black => {state = "".into()},
                                                    TileState::Gray => {state = "absent".into()},
                                                    TileState::Blue => {state = "present".into()},
                                                    TileState::Orange => {state = "correct".into()}
                                                }
                                                new_state = tile.state.clone();
                                            } else if tile.character == *c && tile.state == TileState::Gray && state.is_empty() {
                                                state = "absent".into();
                                                new_state = TileState::Gray;
                                            }

                                        }


                                        html! {
                                            <div class={format!("tile {}", state)} onclick={Callback::from(move |e: MouseEvent| {
                                                e.prevent_default();

                                                let tile_state = new_state.clone();
                                                callback.emit(Msg::UpdateTile(
                                                    Tile { state: tile_state, position: index, character },
                                                    board_index
                                                ));
                                            })}>
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
