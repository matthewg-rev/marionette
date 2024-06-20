use dioxus::prelude::*;
use dioxus_html_macro::*;

pub fn random_message() -> String {
    let messages = vec![
        "Oops! Looks like the program counter went past the breakpoint?",
        "Looks like you changed the RIP to an outside location!",
        "Uh oh! The content you're looking for is not here!",
        "Looks like you're trying to access a non-existent memory location!",
    ];

    let random_index = rand::random::<usize>() % messages.len();
    messages[random_index].to_string()
}

#[component]
pub fn NotFound() -> Element {
    rsx! {
        style { {include_str!("resources/styles/page-not-found/page-not-found.css")} },
        h1 {
            id: "page_not_found_message",
            "404"
        },
        p {
            id: "pnf_text",
            {random_message()}
        }
    }
}