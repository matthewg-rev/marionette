use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::prelude::*;
use dioxus_router::{use_route, use_router};
use dioxus_desktop::{LogicalSize, use_window};
use dioxus_html_macro::*;
use crate::states::explorer::{ExplorerState, FileEntry};
use crate::states::selector::SelectorState;

use crate::on_result;
use crate::msgbox::{Msg, MsgButtons, MsgResult, MsgSize, MsgType};

pub fn icon_info_from_file(file: FileEntry) -> (String, String, String, String, String) {
    let lookup: HashMap<&str, (&str, &str, &str, &str, &str)> = HashMap::from([
        ("Folder", ("NerdFontsSymbols Nerd Font", "", "#dcb67a", "1.2em", "")),
        ("zip", ("NerdFontsSymbols Nerd Font", "", "#1b5eab", "1.2em", "")),

        ("exe", ("NerdFontsSymbols Nerd Font", "󰘔", "#bebebe", "1.2em", "")),
        ("dll", ("NerdFontsSymbols Nerd Font", "", "#bebebe", "1.2em", "")),

        ("lua", ("NerdFontsSymbols Nerd Font", "󰢱", "#0095d7", "1.2em", "")),
        ("js", ("NerdFontsSymbols Nerd Font", "󰌞", "#f5de19", "1.2em", "")),
        ("py", ("NerdFontsSymbols Nerd Font", "󰌠", "#3774a6", "1.2em", "")),
        ("css", ("NerdFontsSymbols Nerd Font", "", "#2d9fd5", "1.2em", "")),
        ("html", ("NerdFontsSymbols Nerd Font", "󰌝", "#e5633f", "1.2em", "")),

        ("json", ("NerdFontsSymbols Nerd Font", "", "#f5de19", "1.2em", "")),
        ("sh", ("NerdFontsSymbols Nerd Font", "", "#f5de19", "1.2em", "")),

        ("key", ("NerdFontsSymbols Nerd Font", "", "#bebebe", "1.2em", "")),
        ("conf", ("NerdFontsSymbols Nerd Font", "", "#bebebe", "1.2em", "")),
        ("txt", ("NerdFontsSymbols Nerd Font", "󰧮", "#bebebe", "1.2em", "")),

        ("ico", ("NerdFontsSymbols Nerd Font", "󰺰", "#bebebe", "1.2em", "")),
        ("jpg", ("NerdFontsSymbols Nerd Font", "󰺰", "#bebebe", "1.2em", "")),
        ("png", ("NerdFontsSymbols Nerd Font", "󰺰", "#bebebe", "1.2em", ""))
    ]);

    if let Some(icon_info) = lookup.get(file.file_extension.as_str()) {
        return (icon_info.0.to_string(), icon_info.1.to_string(), icon_info.2.to_string(), icon_info.3.to_string(), icon_info.4.to_string());
    }

    (String::from("NerdFontsSymbols Nerd Font"), String::from("󰈤"), String::from("#bebebe"), String::from("1.2em"), String::from(""))
}

pub fn projects_container(cx: Scope) -> Element {
    let window = use_window(cx);
    let router = use_router(cx);
    let selector_state = use_shared_state::<SelectorState>(cx).unwrap();

    let projects = html!(
        <div class="projects_grid_container">
            {
                /*database_controller.read().get_project_databases().iter().map(|proj| {
                    let db = proj.clone();
                    let path = proj.database.path.clone();
                    html!(
                        <div class="project_container" onclick = {move |_| {
                            let active_controller = database_controller.read();
                            let compatibility_report = active_controller.checker.check_compatibility(db.clone().database);
                            if !compatibility_report.compatible() {
                                if compatibility_report.has_errors() {
                                    Msg::new(
                                        format!(
                                            "{}",
                                            compatibility_report
                                        ),
                                        "Cannot open Project".to_string(),
                                        MsgType::Error,
                                        MsgButtons::Ok,
                                        MsgSize::Fit
                                    ).display(window);
                                } else {
                                    let mut msgbox = Msg::new(
                                        format!(
                                            "{}\nAre you sure you want to open the project?",
                                            compatibility_report
                                        ),
                                        "Open Project".to_string(),
                                        MsgType::Warning,
                                        MsgButtons::YesNo,
                                        MsgSize::Fit
                                    );
                                    msgbox.display(window);

                                    let path_str = path.clone();
                                    to_owned![selector_state];
                                    to_owned![router];
                                    on_result!(cx, msgbox, value, {
                                        if value == MsgResult::Yes {
                                            let mut selector_state = selector_state.write();
                                            selector_state.selected_path = path_str;
                                            router.navigate_to("/tool");
                                        }
                                    });
                                }
                            } else {
                                let mut msgbox = Msg::new(
                                    format!(
                                        "{}\nAre you sure you want to open the project?",
                                        compatibility_report
                                    ),
                                    "Open Project".to_string(),
                                    MsgType::Info,
                                    MsgButtons::YesNo,
                                    MsgSize::Fit
                                );
                                msgbox.display(window);

                                let path_str = path.clone();
                                to_owned![selector_state];
                                to_owned![router];
                                on_result!(cx, msgbox, value, {
                                    if value == MsgResult::Yes {
                                        let mut selector_state = selector_state.write();
                                        selector_state.selected_path = path_str;
                                        router.navigate_to("/tool");
                                    }
                                });
                            }
                        }}>
                        <span class="project_icon">""</span>
                        <p id="project_name">{ proj.database.name.clone() }</p>
                    </div>
                )})*/
                html!()
            }
        </div>
    );
    render!(projects)
}

pub fn explorer_container(cx: Scope) -> Element {
    let window = use_window(cx);
    let selector_state = use_shared_state::<SelectorState>(cx).unwrap();

    let refresh_file_explorer = move || {
        let mut explorer_state = selector_state.write();
        let path = explorer_state.explorer_state.full_path();
        if !std::path::Path::new(&path).exists() {
            Msg::new(format!("The path {} does not exist.", path), "Error setting path".to_string(), MsgType::Error, MsgButtons::OkCancel, MsgSize::Fit).display(window);
            return;
        }
        explorer_state.explorer_state.refresh_directory();
    };

    let explorer = html!(
        <div class="file_explorer_top_bar">
            <input class="file_explorer_path" placeholder="Path to folder" value="{selector_state.read().explorer_state.full_path()}" oninput={move |evt| {
                let mut selector_state = selector_state.write();
                selector_state.explorer_state.set_working_directory(evt.value.clone());
            }} onkeydown={move|evt| {
                if evt.code() == Code::Enter {
                    refresh_file_explorer();
                }
            }}/>

            <div class="file_explorer_button" onclick = { move |_| {
                let mut selector_state = selector_state.write();
                selector_state.explorer_state.go_up();
            }}>
                <svg style="width: 20px; height: 15px;" id="go_down_button" view_box="0 0 32 32">
                    <polygon id="svg-data" points="14,16 24,6 25.4,7.4 16.8,16 25.4,24.6 24,26 "/>
                    <rect id="svg-data" x="8" y="4" width="2" height="24"/>
                </svg>
            </div>

            <div class="file_explorer_button" onclick = { move |_| {
                let mut selector_state = selector_state.write();
                selector_state.explorer_state.go_back();
            }}>
                <svg style="width: 20px; height: 15px;" id="go_up_button" view_box="0 0 32 32">
                    <polygon id="svg-data" points="18,16 8,26 6.6,24.6 15.2,16 6.6,7.4 8,6 "/>
                    <rect id="svg-data" x="22" y="4" width="2" height="24"/>
                </svg>
            </div>

            <div class="file_explorer_button" onclick={move |_| {
                    refresh_file_explorer();
                }}>
                <svg style="width: 20px; height: 15px;" id="refresh_button" view_box="0 0 32 32">
                    <path id="svg-data" d="M12,10H6.78A11,11,0,0,1,27,16h2A13,13,0,0,0,6,7.68V4H4v8h8Z"/>
                    <path id="svg-data" d="M20,22h5.22A11,11,0,0,1,5,16H3a13,13,0,0,0,23,8.32V28h2V20H20Z"/>
                </svg>
            </div>
        </div>
        <div class="file_explorer_container">
            {
                {
                    let folders = selector_state.read().explorer_state.current_folders.clone();
                    folders
                }.iter().map(|folder| {
                    let folder_name = folder.file_name.clone();
                    let icon_info = icon_info_from_file(folder.clone());
                    html!(
                        <div id="file_entry" ondblclick={move |_| {
                            let mut selector_state = selector_state.write();
                            selector_state.explorer_state.go_into_dir(folder_name.clone());
                        }}>
                            <span style=r#"
                                padding-left: 5px;
                                font-size: {icon_info.3.clone()};
                                font-family: {icon_info.0.clone()};
                                color: {icon_info.2.clone()};
                                {icon_info.4.clone()}"#>
                                {icon_info.1.clone()}
                            </span>
                            <span>{folder.file_name.clone()}</span>
                            <span>{folder.file_size.clone()}</span>
                            <span>{folder.date_modified.clone()}</span>
                        </div>
                    )
                })
            }
            {
                {
                    let files = selector_state.read().explorer_state.current_files.clone();
                    files
                }.iter().map(|file| {
                    let file_path = file.file_path.clone();
                    let icon_info = icon_info_from_file(file.clone());
                    html!(
                        <div id="file_entry" onclick={move |_| {
                            let mut selector_state = selector_state.write();
                            selector_state.selected_path = file_path.clone();
                        }}>
                            <span style="padding-left: 5px; font-size: {icon_info.3.clone()}; font-family: {icon_info.0.clone()}; color: {icon_info.2.clone()}; {icon_info.4.clone()}">{icon_info.1.clone()}</span>
                            <span>{file.file_name.clone()}</span>
                            <span>{file.file_size.clone()}</span>
                            <span>{file.date_modified.clone()}</span>
                        </div>
                    )
                })
            }
        </div>
    );
    render!(explorer)
}

pub fn bottom_bar_container(cx: Scope) -> Element {
    let router = use_router(cx);
    let selector_state = use_shared_state::<SelectorState>(cx).unwrap();

    let bottom_bar = html!(
        <div class="bottom_bar">
            <input
                class="input_box"
                placeholder="Path to file"
                value="{selector_state.read().selected_path}"
                oninput={ move |evt| {
                    let mut selector_state = selector_state.write();
                    selector_state.selected_path = evt.value.clone();
                }
            }/>

            <div
                class="bottom_button"
                onclick={ move |_| {
                    router.navigate_to("/")
                }
            }>"Cancel"</div>

            <div
                class="bottom_button"
                onclick={ move |_| {
                    router.navigate_to("/tool");
                }
            }>"Analyze"</div>
        </div>
    );
    render!(bottom_bar)
}

pub fn header_container(cx: Scope) -> Element {
    let router = use_router(cx);
    let route = use_route(cx);
    let tab_str = route.segment("tab").unwrap_or("none");

    let header = html!(
        <div class="header">
            <div id={
                if tab_str == "none" { "underline_important_text" } else { "" }
            } class="tab_button" onclick={
                move |_| {
                    router.navigate_to("/open/");
                }
            }>"Analysis Selector"</div>

            <div id="tab_container">
                <div id={
                    if tab_str == "projects" { "underline_important_text" } else { "" }
                } class="tab_button" onclick={
                    move |_| {
                        router.navigate_to("/open/projects");
                    }
                }>"Projects"</div>

                <div id={
                    if tab_str == "files" { "underline_important_text" } else { "" }
                } class="tab_button" onclick={
                    move |_| {
                        router.navigate_to("/open/files");
                    }
                }>"Files"</div>
            </div>
        </div>
    );
    render!(header)
}

pub fn analysis_selector(cx: Scope) -> Element {
    let window = use_window(cx);
    window.set_title("Marionette Analysis Selector");
    window.set_resizable(true);
    window.set_min_inner_size(Some(LogicalSize::new(450, 300)));

    let route = use_route(cx);
    let tab_str = route.segment("tab").unwrap_or("none");

    cx.render(rsx!(
        style { include_str!("resources/styles/analysis-selector/analysis-selector.css") }
        html!(
            { rsx! { header_container {} } }
            {
                match tab_str {
                    "projects" => rsx! { projects_container {} },
                    "files" => rsx! { explorer_container {} },
                    _ => html!(
                        <div class="repeating_diagonal_lines"></div>
                    )
                }
            }
            { rsx! { bottom_bar_container {} } }
        )
    ))
}