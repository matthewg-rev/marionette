use dioxus::prelude::*;
use dioxus_router::{Link, Router, Route, Redirect};
use dioxus_desktop::{Config, LogicalSize, use_window, WindowBuilder};
use dioxus_html_macro::*;

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
    if let Ok(latest_release) = latest_release {
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
                        <p id="release-title">{latest_release.tag_name + " - " + latest_release.name.unwrap_or("".to_string()).as_str()}</p>
                        <p id="release-description">{latest_release.body.unwrap()[0..35].to_string() + "..."}</p>
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
            <div class="tool-button-icon">
                <svg xmlns="http://www.w3.org/2000/svg" style="width: 32px; height: 32px; viewBox: 0 0 32 32; fill:#fff;">
                    <path id="svg-data" d="M28.5039,8.1362l-12-7a1,1,0,0,0-1.0078,0l-12,7A1,1,0,0,0,3,9V23a1,1,0,0,0,.4961.8638l12,7a1,1,0,0,0,1.0078,0l12-7A1,1,0,0,0,29,23V9A1,1,0,0,0,28.5039,8.1362ZM16,3.1577,26.0156,9,16,14.8423,5.9844,9ZM5,10.7412l10,5.833V28.2588L5,22.4258ZM17,28.2588V16.5742l10-5.833V22.4258Z"/>
                </svg>
            </div>
            <p id="button_description">"Launch Analysis"</p>
        </div>
    ))
}

fn launch_empty_element(cx: Scope) -> Element {
    cx.render(html!(
        <div class="tool-button-container" onclick={move |_| {
                let router = dioxus_router::use_router(cx);
                router.navigate_to("/tool");
            }}>
            <div class="tool-button-icon">
                <svg xmlns="http://www.w3.org/2000/svg" style="width: 32px; height: 32px; viewBox: 0 0 32 32; fill:#fff;">
                    <path id="svg-data" d="M26,28H6a2.0027,2.0027,0,0,1-2-2V6A2.0027,2.0027,0,0,1,6,4H16V6H6V26H26V16h2V26A2.0027,2.0027,0,0,1,26,28Z"/>
                    <polygon id="svg-data" points="20 2 20 4 26.586 4 18 12.586 19.414 14 28 5.414 28 12 30 12 30 2 20 2"/>
                </svg>
            </div>
            <p id="button_description">"Launch Empty"</p>
        </div>
    ))
}

pub fn welcome_page(cx: Scope) -> Element {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let outdated = is_outdated();
    let window = use_window(cx);
    window.set_resizable(false);
    window.set_inner_size(LogicalSize::new(550, 400));
    cx.render(
        rsx!(
            style { include_str!("resources/styles/welcome-style.css") },
            html!(
                <h1 id="welcome_message">"Marionette"</h1>
                <p id="descriptor">
                    <span>"Marionette is a disassembler "</span>
                    <span id="underline_important_text">"study tool"</span>
                    <span>" for reverse-engineering."</span>
                </p>
                {
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
                }
            ),
            if outdated { latest_commit_element(cx) } else { html!() },
            html!(
                <p id="version">"Version "{VERSION}""</p>
            )
        )
    )
}