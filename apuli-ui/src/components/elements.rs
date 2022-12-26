use crate::Msg;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ToggleButtonProps {
    pub callback: Callback<Msg>,
}

#[function_component(ToggleButton)]
pub fn toggle_button(props: &ToggleButtonProps) -> yew::Html {
    let callback = props.callback.clone();
    
    html! {
        <>
        <div>
            {{
                let onmousedown = Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                callback.emit(Msg::Enter(true));
                });
                html! {
                <button data-nosnippet="" class={classes!("keyboard-button", "keyboard-button-submit", "correct")}
                                    onmousedown={onmousedown}>
                    { "VALMIS" }
                </button>
                }
            }}
        </div>
        </>
    
    }
    
}
