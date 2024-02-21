use dioxus::prelude::*;
use dioxus_router::{Link, Router, Route, Redirect};
use dioxus_desktop::{Config, LogicalSize, use_window, WindowBuilder};
use dioxus_html_macro::*;

use crate::msgbox::{Msg, MsgButtons, MsgResult, MsgSize, MsgType};

fn is_outdated() -> bool {
    let octocrab = octocrab::instance();
    let repo_handler = octocrab.repos("matthewg-rev", "marionette");
    let releases_handler = repo_handler.releases();
    let latest_release = releases_handler.get_latest();
    let latest_release = tokio::runtime::Runtime::new().unwrap().block_on(latest_release);
    if let Ok(latest_release) = latest_release {
        let latest_version = latest_release.tag_name;
        let current_version = env!("CARGO_PKG_VERSION");
        if latest_version != current_version {
            return true;
        }
    }
    false
}

fn latest_commit_element(cx: Scope) -> LazyNodes {
    let octocrab = octocrab::instance();
    let repo_handler = octocrab.repos("matthewg-rev", "marionette");
    let releases_handler = repo_handler.releases();
    let latest_release = releases_handler.get_latest();
    let latest_release = tokio::runtime::Runtime::new().unwrap().block_on(latest_release);
    let window = use_window(cx);

    if let Ok(latest_release) = latest_release {
        let latest_version = latest_release.clone().tag_name;
        let latest_body = latest_release.clone().body.clone().unwrap();
        let latest_body_truncated = if latest_body.len() > 33 {
            latest_body[..33].to_string() + "..."
        } else {
            latest_body.clone()
        };

        let mut msgbox = Msg::new(
            format!(
                "Marionette has updated:\n{}",
                latest_body
            ),
            format!("Version {} is available!", latest_version),
            MsgType::Info,
            MsgButtons::Ok,
            MsgSize::Fit
        );
        msgbox.display(window);

        return html!(
            <div class="row">
                <div class="git-release-container" onclick={ move |_| {
                        let _ = open::that(latest_release.html_url.as_str());
                    }}>
                    <div style="margin-left: 15px; align-items: center;">
                        <div id="svg-container" style="width: 20px; height: 20px;">
                            { rsx! (
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    style: "width: 100%; height: 100%; fill: #B2B2B2FF;",
                                    view_box: "0 0 32 32",
                                    filter: "drop-shadow(0px 0px 5px rgb(0 0 0 / 0.4))",
                                    html!(
                                        <path id="svg-data" d="M10,14a4,4,0,1,1,4-4A4.0045,4.0045,0,0,1,10,14Zm0-6a2,2,0,1,0,1.998,2.0044A2.002,2.002,0,0,0,10,8Z"></path>
                                        <path id="svg-data" d="M16.6436,29.4145,2.5858,15.3555A2,2,0,0,1,2,13.9414V4A2,2,0,0,1,4,2h9.9413a2,2,0,0,1,1.4142.5858L29.4144,16.6436a2.0005,2.0005,0,0,1,0,2.8285l-9.9424,9.9425a2.0008,2.0008,0,0,1-2.8285,0ZM4,4v9.9417L18.0578,28,28,18.0579,13.9416,4Z"></path>
                                    )
                                }
                            ) }
                        </div>
                    </div>
                    <div class="column" style="margin-left: 25px; margin-right: 25px;">
                        <p id="release-title">{latest_version + " - " + latest_release.name.unwrap_or("".to_string()).as_str()}</p>
                        <p id="release-description">{latest_body_truncated}</p>
                    </div>
                </div>
            </div>
        )
    }
    html!()
}

fn launch_analysis_element(cx: Scope) -> Element {
    cx.render(html!(
        <div class="tool-button-container" onclick={move |_| {
                let router = dioxus_router::use_router(cx);
                router.navigate_to("/open");
            }}>
            <p id="button_header">"Launch Analysis"</p>
            <p id="button_description">"Open a file for analysis or select a previously started project"</p>
        </div>
    ))
}

fn launch_empty_element(cx: Scope) -> Element {
    cx.render(html!(
        <div class="tool-button-container" onclick={move |_| {
                let router = dioxus_router::use_router(cx);
                router.navigate_to("/tool");
            }}>
            <p id="button_header">"Launch Empty"</p>
            <p id="button_description">"Open an empty project"</p>
        </div>
    ))
}

pub fn welcome_page(cx: Scope) -> Element {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let outdated = is_outdated();
    let window = use_window(cx);

    window.set_resizable(false);
    window.set_inner_size(LogicalSize::new(825, 500));
    cx.render(
        rsx!(
            style { include_str!("resources/styles/welcome/welcome.css") },
            script { include_str!("resources/scripts/welcome.js") },
            html!(
                <h1 id="welcome-message">"Marionette"</h1>
                <p id="descriptor">
                    <span>"Marionette is a disassembler "</span>
                    <span id="underline-important_text">"study tool"</span>
                    <span>" for reverse-engineering."</span>
                </p>

                <div class="container">
                    <div class="row">
                        { rsx!(launch_analysis_element {}) }
                        { rsx!(launch_empty_element {}) }
                    </div>
                </div>
                
                /*{
                    if outdated {
                        html!(
                            <div class="container">
                                <div class="row">
                                    { rsx!(launch_analysis_element {}) }
                                    { rsx!(launch_empty_element {}) }
                                </div>
                            </div>
                        )
                    } else {
                        html!(
                            <div class="container" style="display: grid; height: 100%;">
                                <div class="row" style="height: 100%;">
                                    { rsx!(launch_analysis_element {}) }
                                    { rsx!(launch_empty_element {}) }
                                </div>
                            </div>
                        )
                    }
                    if outdated { latest_commit_element(cx) } else { html!() },
                }*/
            ),
            html!(
                <p id="version">"Version "{VERSION}""</p>
            )
        )
    )
}