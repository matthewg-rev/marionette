use dioxus::prelude::*;
use dioxus_html_macro::*;

#[component]
pub fn NotFound() -> Element {
    rsx! {
        style { {include_str!("resources/styles/page-not-found/page-not-found.css")} },
        {html!(
            <h1 id="page_not_found_message">"404"</h1>
            <p id="pnf_text">"Oops! Looks like the program counter went past the breakpoint?"</p>
        )}
    }
}