mod welcome;
mod page_not_found;
mod selector_service;
mod tool;
mod msgbox;

use dioxus::prelude::*;
use dioxus_html_macro::html;
use dioxus_router::{Router, Route};
use dioxus_desktop::{Config, use_window, WindowBuilder};

fn main() {
    dioxus_desktop::launch_cfg(
        portal,
        Config::default().with_window(
            WindowBuilder::new()
                .with_title("Marionette")
        )
    );
}

fn portal(cx: Scope) -> Element {
    let selector_state = selector_service::SelectorState::new();
    use_shared_state_provider(cx, || selector_state);

    cx.render(rsx!(
        html!(<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200" />)
        style { include_str!("resources/styles/file-icons.min.css") },
        Router {
            Route { to: "/", welcome::welcome_page {} },
            Route { to: "/open", selector_service::analysis_selector { } },
            Route { to: "/open/:tab", selector_service::analysis_selector { } },
            Route { to: "/tool", tool::tool_page {} },
            Route { to: "", page_not_found::page_not_found {} },
        }
    ))
}