mod welcome;
mod page_not_found;
mod selector_service;
mod tool;
mod msgbox;
mod states;
mod plugin;

use std::fs;
use std::env;
use futures::{executor, FutureExt};
use dioxus::{html::p, prelude::*};
use dioxus::desktop::{Config, WindowBuilder};

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
    let launcher = LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_menu(None)
                .with_window(
                    WindowBuilder::new()
                        .with_always_on_top(false)
                        .with_resizable(false)
                        .with_title("Marionette")
                )
        ).launch(portal);
    launch(portal);
}

#[component]
fn portal() -> Element {
    let future = use_resource(move || async move {
        let mut evaluator = eval(include_str!("resources/scripts/interop_connector.js"));
        let res = evaluator.await;
        res
    });

    let mut plugins_vec = vec![];
    let current_exe = env::current_exe().unwrap();
    let mut plugin_dir = current_exe.parent().unwrap().to_path_buf();
    plugin_dir.push("plugins");
    let plugins_dir = fs::read_dir(plugin_dir).unwrap();
    for entry in plugins_dir {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension().unwrap_or_default();
        if extension != "py" {
            continue;
        }

        let path = path.to_str().unwrap();
        let plugin = plugin::Plugin::new(path);
        match plugin {
            Ok(plugin) => { 
                let result = plugin.init();
                plugins_vec.push(plugin);
            },
            Err(_) => continue,
        }
    }

    let plugins = use_context_provider(|| Signal::new(plugins_vec));
    let selector_state = use_context_provider(|| Signal::new(states::selector::SelectorState::new()));

    rsx!(
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
        },
        style { {include_str!("resources/styles/misc/file-icons.min.css")} },
        script { {include_str!("resources/scripts/interop.js")} },
        Router::<Route> { }
        {
            future.value();
        }
    )
}