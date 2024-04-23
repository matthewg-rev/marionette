mod welcome;
mod page_not_found;
mod selector_service;
mod tool;
mod msgbox;
mod states;

use dioxus::prelude::*;
use dioxus_html_macro::html;
use dioxus_router::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use crate::{
    welcome::Welcome,
    selector_service::OpenTab,
    tool::Tool,
    page_not_found::NotFound,
};

#[derive(Clone, Routable)]
pub enum Route {
    #[route("/")]
    Welcome {},

    #[route("/open/:tab")]
    OpenTab { tab: usize },
    
    #[route("/tool")]
    Tool {},

    #[route("")]
    NotFound {},
}

fn main() {
    dioxus_desktop::launch_cfg(
        portal,
        Config::default().with_window(
            WindowBuilder::new()
                .with_title("Marionette")
        )
    );
}

#[component]
fn portal(cx: Scope) -> Element {
    let selector_state = states::selector::SelectorState::new();
    use_shared_state_provider(cx, || selector_state);

    cx.render(rsx!(
        html!(<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />),
        style { include_str!("resources/styles/misc/file-icons.min.css") },
        Router::<Route> { }
    ))
}