use dioxus::prelude::*;
use dioxus_html_macro::html;
use crate::selector_service::SelectorState;

pub fn tool_page(cx: Scope) -> Element {
    let selector_state = use_shared_state::<SelectorState>(cx).unwrap();
    let selected = selector_state.read().selected_path.clone();

    cx.render(rsx!(
        style { include_str!("resources/styles/tool.css") }
        html!()
    ))
}