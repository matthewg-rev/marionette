mod welcome;
mod page_not_found;
mod selector_service;
mod tool;
mod msgbox;
mod states;
mod plugin;

use std::fs;
use std::env;
use dioxus::desktop::tao::platform::windows::WindowBuilderExtWindows;
use marionette_core::lexer_service::LexerService;
use serde_json::{json, Value};
use futures::{executor, FutureExt};
use dioxus::{
    html::p, prelude::*
};
use marionette_core::general_lexer::GeneralLexer;
use dioxus::desktop::{
    Config,
    WindowBuilder
};

use crate::{
    welcome::Welcome, 
    selector_service::OpenTab, 
    tool::Tool, 
    page_not_found::NotFound
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

fn dispatch(method: String, data: Value) -> Result<String, String> {
    let mut result = "".to_string();

    result = match method.as_str() {
        "lex" => {
            let content = data["text"].as_str().unwrap().to_string();
            let lexer_choice = data["lexer"].as_str().unwrap().to_string();

            let result = LexerService::lex(content, lexer_choice);
            if result.is_err() {
                return Err(result.err().unwrap().to_string());
            }

            result.unwrap().to_string()
        }
        _ => format!("Error, no method found: {}", method)
    };

    Ok(result)
}

fn respond(response: Value, eval: UseEval) {
    if eval.send(response.to_string().into()).is_ok() { 
        return;
    }
    
    println!("Error sending message: {:?}", response.to_string());
    println!("User interface is likely frozen or disconnected, exiting...");
    std::process::exit(1);
}

fn main() {
    let windowBuilder = WindowBuilder::new()
        .with_always_on_top(false)
        .with_resizable(false)
        .with_title("Marionette");

    let config = dioxus::desktop::Config::new().with_window(windowBuilder).with_menu(None);
    let launcher = LaunchBuilder::new().with_cfg(config);
    launcher.launch(portal);
}

#[component]
fn portal() -> Element {
    let mut eval = eval(include_str!("resources/scripts/interop_connector.js"));
    
    let task = use_future(move || {
        async move {
            loop {
                to_owned![eval];
                if let Ok(message) = eval.recv().await {
                    let mut response = json!({
                        "status": "ok",
                        "data": ""
                    });

                    if let Some(message) = message.as_object() {
                        if message.contains_key("method") && message.contains_key("data") {
                            let res = dispatch(
                                message["method"].as_str().unwrap().to_string(), 
                                Value::Object(message["data"].as_object().unwrap().clone())
                            );

                            if let Ok(res) = res {
                                response["data"] = res.clone().into();
                                respond(response, eval);
                            } else {
                                response["status"] = "err".into();
                                response["data"] = res.clone().err().unwrap().into();
                                respond(response, eval);
                            }
                        }
                    } else {
                        response["status"] = "err".into();
                        response["data"] = format!("Error: {}", message).into();
                        respond(response, eval);
                    }
                }
            }
        }
    });

    let mut plugins_vec = vec![];
    let current_exe = env::current_exe().unwrap();
    let mut plugin_dir = current_exe.parent().unwrap().to_path_buf();
    plugin_dir.push("plugins");
    let plugins_dir = fs::read_dir(plugin_dir);

    if let Ok(plugins_dir) = plugins_dir {
        for entry in plugins_dir.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension() == Some(std::ffi::OsStr::new("py")) {
                if let Ok(plugin) = plugin::Plugin::new(path.to_str().unwrap()) {
                    plugin.init();
                    plugins_vec.push(plugin);
                }
            }
        }
    }

    let plugins = use_context_provider(|| Signal::new(plugins_vec));
    let selector_state = use_context_provider(|| Signal::new(states::selector::SelectorState::new()));

    rsx!(
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
        }
        style { {include_str!("resources/styles/misc/file-icons.min.css")} }
        script { {include_str!("resources/scripts/interop.js")} }
        Router::<Route> { }
    )
}