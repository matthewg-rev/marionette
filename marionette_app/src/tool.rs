use dioxus::prelude::*;
use dioxus_html_macro::html;
use dioxus_desktop::{use_window};
use crate::states::selector::SelectorState;

pub fn tool_page(cx: Scope) -> Element {
    let window = use_window(cx);
    window.set_resizable(true);

    let selector_state = use_shared_state::<SelectorState>(cx).unwrap();
    let selected = selector_state.read().selected_path.clone();

    cx.render(rsx!(
        style { include_str!("resources/styles/tool.css") },

        script { include_str!("resources/scripts/tool/widget.js") },
        script { include_str!("resources/scripts/tool/widgets/clock.js") },

        script { include_str!("resources/scripts/tool/canvas.js") },
        script { include_str!("resources/scripts/tool/tool.js") },
        html!(
            <header></header>
            <div class="canvas">
            </div>
        )
    ))
}