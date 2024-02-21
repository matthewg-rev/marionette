use dioxus::prelude::*;
use dioxus_html_macro::*;

pub fn page_not_found(cx: Scope) -> Element {
    cx.render(rsx!(
        style { include_str!("resources/styles/page-not-found/page-not-found.css") },
        html!(
            <h1 id="page_not_found_message">"404"</h1>
            <p id="pnf_text">"Oops! Looks like the program counter went past the breakpoint?"</p>
        )
    ))
}