#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus::desktop::{use_window, LogicalSize, DesktopService};
use dioxus_html_macro::*;

use crate::on_result;
use crate::states::explorer::{ExplorerState, FileEntry};
use crate::states::selector::SelectorState;

use crate::msgbox::{Msg, MsgButtons, MsgResult, MsgType};

static ICONS: &[(&str, (&str, &str, &str, &str, &str))] = &[
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
];

#[derive(PartialEq, Props, Clone)]
struct TabProps {
    text: &'static str,
    tab: usize
}

pub fn icon_info_from_file(file: FileEntry) -> (String, String, String, String, String) {
    let search = ICONS.binary_search_by(
        |(ext, _) | ext.cmp(&file.file_extension.as_str())
    ).map(|i| ICONS[i].1);
    
    if let Ok(icon) = search {
        return (icon.0.to_string(), icon.1.to_string(), icon.2.to_string(), icon.3.to_string(), icon.4.to_string());
    }

    (String::from("NerdFontsSymbols Nerd Font"), String::from("󰈤"), String::from("#bebebe"), String::from("1.2em"), String::from(""))
}

pub fn projects_container() -> Element {
    let window = use_window();
    let router = router();
    let selector_state = use_context::<Signal<SelectorState>>();

    /*let mut msgbox = Msg::new(
        format!("Testing testing 123 awesome sauce!"),
        "Open Project".to_string(),
        MsgType::Warning,
        MsgButtons::YesNo
    );
    msgbox.display(&window);

    on_result!(msgbox, value, {
        if value == MsgResult::Yes {
            let mut msgbox = Msg::new(
                format!(
                    "lol nice",
                ),
                "Open Project".to_string(),
                MsgType::Info,
                MsgButtons::YesNo
            );
            msgbox.display(&window);
        }
    });*/

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
    
    rsx!{
        {projects}
    }
}

pub fn refresh_file_explorer(window: Rc<DesktopService>, selector_state: Signal<SelectorState>) {
    let mut explorer_state = selector_state;
    let path = explorer_state.read().explorer_state.full_path();
    if !std::path::Path::new(&path).exists() {
        let msg = Msg::new(
            format!("The path {} does not exist.", path), 
            "Error setting path".to_string(), 
            MsgType::Error, 
            MsgButtons::OkCancel
        );
        msg.display(&window);
    }
    explorer_state.write().explorer_state.refresh_directory();
}

#[derive(PartialEq, Props, Clone)]
pub struct FileExplorerPathInputProps {
    pub path: String,
    pub placeholder: &'static str,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

pub fn FileExplorerPathInput(props: FileExplorerPathInputProps) -> Element {
    rsx! {
        input {
            class: "file_explorer_path",
            placeholder: props.placeholder,
            value: props.path,
            onkeydown: move |evt| props.onkeydown.call(evt)
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct FileExplorerButtonProps {
    pub onclick: EventHandler<MouseEvent>,
    pub children: Element,
}

pub fn FileExplorerButton(props: FileExplorerButtonProps) -> Element {
    rsx! {
        div {
            class: "file_explorer_button",
            onclick: move |evt| props.onclick.call(evt),
            {props.children}
        }
    }
}

pub fn GoDownButtonSvg() -> Element {
    rsx! {
        svg {
            id: "go_down_button",
            width: "20px",
            height: "15px",
            view_box: "0 0 32 32",

            polygon {
                id: "svg-data",
                points: "14,16 24,6 25.4,7.4 16.8,16 25.4,24.6 24,26 "
            }

            rect {
                id: "svg-data",
                x: "8",
                y: "4",
                width: "2",
                height: "24"
            }
        }
    }
}

pub fn GoUpButtonSvg() -> Element {
    rsx! {
        svg {
            id: "go_up_button",
            width: "20px",
            height: "15px",
            view_box: "0 0 32 32",

            polygon {
                id: "svg-data",
                points: "18,16 8,26 6.6,24.6 15.2,16 6.6,7.4 8,6 "
            }

            rect {
                id: "svg-data",
                x: "22",
                y: "4",
                width: "2",
                height: "24"
            }
        }
    }
}

pub fn RefreshButtonSvg() -> Element {
    rsx! {
        svg {
            id: "refresh_button",
            width: "20px",
            height: "15px",
            view_box: "0 0 32 32",

            path {
                id: "svg-data",
                d: "M12,10H6.78A11,11,0,0,1,27,16h2A13,13,0,0,0,6,7.68V4H4v8h8Z"
            }

            path {
                id: "svg-data",
                d: "M20,22h5.22A11,11,0,0,1,5,16H3a13,13,0,0,0,23,8.32V28h2V20H20Z"
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ExplorerFolderProps {
    pub icon_info: (String, String, String, String, String),
    pub folder_name: String,
    pub folder_size: String,
    pub date_modified: String
}

pub fn ExplorerFolder(props: ExplorerFolderProps) -> Element {
    let mut selector_state = use_context::<Signal<SelectorState>>();
    rsx! {
        div {
            id: "file_entry",
            ondoubleclick: move |_| {
                let mut selector_state = selector_state.write();
                selector_state.explorer_state.go_into_dir(props.folder_name.clone());
            },

            span {
                style: r#"
                    padding-left: 5px;
                    font-size: {props.icon_info.3.clone()};
                    font-family: {props.icon_info.0.clone()};
                    color: {props.icon_info.2.clone()};
                    {props.icon_info.4.clone()}"#,
                {props.icon_info.1.clone()}
            }
            span { { props.folder_name.clone() } }
            span { { props.folder_size.clone() } }
            span { { props.date_modified.clone() } }
        }
    }
}

pub fn ExplorerFolders() -> Element {
    let mut selector_state = use_context::<Signal<SelectorState>>();
    let folders = selector_state.read().explorer_state.current_folders.clone();

    rsx! {
        {folders.iter().map(|folder| {
            let folder_name = folder.file_name.clone();
            let icon_info = icon_info_from_file(folder.clone());
            rsx! {
                ExplorerFolder {
                    icon_info: icon_info,
                    folder_name: folder_name.clone(),
                    folder_size: folder.file_size.clone(),
                    date_modified: folder.date_modified.clone()
                }
            }
        })}
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ExplorerFileProps {
    pub icon_info: (String, String, String, String, String),
    pub file_name: String,
    pub file_size: String,
    pub date_modified: String
}

pub fn ExplorerFile(props: ExplorerFileProps) -> Element {
    let mut selector_state = use_context::<Signal<SelectorState>>();
    rsx! {
        div {
            id: "file_entry",
            onclick: move |_| {
                let mut selector_state = selector_state.write();
                selector_state.selected_path = props.file_name.clone();
            },

            span {
                style: r#"
                    padding-left: 5px;
                    font-size: {props.icon_info.3.clone()};
                    font-family: {props.icon_info.0.clone()};
                    color: {props.icon_info.2.clone()};
                    {props.icon_info.4.clone()}"#,
                {props.icon_info.1.clone()}
            }
            span { { props.file_name.clone() } }
            span { { props.file_size.clone() } }
            span { { props.date_modified.clone() } }
        }
    }
}

pub fn ExplorerFiles() -> Element {
    let mut selector_state = use_context::<Signal<SelectorState>>();
    let files = selector_state.read().explorer_state.current_files.clone();

    rsx! {
        {files.iter().map(|file| {
            let file_path = file.file_path.clone();
            let icon_info = icon_info_from_file(file.clone());
            rsx! {
                ExplorerFile {
                    icon_info: icon_info,
                    file_name: file.file_name.clone(),
                    file_size: file.file_size.clone(),
                    date_modified: file.date_modified.clone()
                }
            }
        })}
    }
}

pub fn explorer_container() -> Element {
    let mut selector_state = use_context::<Signal<SelectorState>>();

    rsx! {
        div {
            class: "file_explorer_top_bar",

            FileExplorerPathInput {
                path: selector_state.read().explorer_state.full_path(),
                placeholder: "Path to folder",
                onkeydown: move |evt: KeyboardEvent| {
                    if evt.code() == Code::Enter {
                        let window = use_window();
                        refresh_file_explorer(window.clone(), selector_state.clone());
                    }
                }
            }

            FileExplorerButton {
                onclick: move |_| {
                    let mut selector_state = selector_state.write();
                    selector_state.explorer_state.go_up();
                },
                GoDownButtonSvg {}
            }

            FileExplorerButton {
                onclick: move |_| {
                    let mut selector_state = selector_state.write();
                    selector_state.explorer_state.go_back();
                },
                GoUpButtonSvg {}
            }

            FileExplorerButton {
                onclick: move |_| {
                    let window = use_window();
                    refresh_file_explorer(window.clone(), selector_state.clone());
                },
                RefreshButtonSvg {}
            }
        }

        div {
            class: "file_explorer_container",
            ExplorerFolders {}
            ExplorerFiles {}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct BottomBarInputProps {
    pub text: &'static str,
    pub value: String,
    pub oninput: EventHandler<FormEvent>,
}

pub fn BottomBarInput(props: BottomBarInputProps) -> Element {
    rsx! {
        input {
            class: "input_box",
            placeholder: props.text,
            value: props.value,
            oninput: move |evt| props.oninput.call(evt)
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct BottomBarButtonProps {
    pub text: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

pub fn BottomBarButton(props: BottomBarButtonProps) -> Element {
    rsx! {
        div {
            class: "bottom_button",
            onclick: move |evt| props.onclick.call(evt),
            {props.text}
        }
    }
}

pub fn bottom_bar_container() -> Element {
    let mut selector_state = use_context::<Signal<SelectorState>>();
    let navigator = use_navigator();

    rsx! { 
        div {
            class: "bottom_bar",
            BottomBarInput {
                text: "Path to file",
                value: selector_state.read().selected_path.clone(),
                oninput: move |evt: FormEvent| {
                    let mut selector_state = selector_state.write();
                    selector_state.selected_path = evt.value().clone();
                }
            }
            BottomBarButton {
                text: "Cancel",
                onclick: move |_| {
                    navigator.push(crate::Route::Welcome {});
                }
            }
            BottomBarButton {
                text: "Analyze",
                onclick: move |_| {
                    navigator.push(crate::Route::Tool {});
                }
            }
        }
    }
}

pub fn HeaderTabButton(props: TabProps) -> Element {
    let route = use_route::<crate::Route>();
    let tab_str = match route {
        crate::Route::OpenTab { tab } => tab,
        _ => 0
    };

    html! {
        <div id={if tab_str == props.tab { "underline_important_text" } else { "" }} class="tab_button" onclick={move |_| {
            let navigator = use_navigator();
            navigator.push(crate::Route::OpenTab { tab: props.tab });
        }}>{props.text}</div>
    }
}

pub fn header_container() -> Element {
    rsx! {
        div {
            class: "header",
            HeaderTabButton { text: "Analysis Selector", tab: 0 }

            div {
                id: "tab_container",
                HeaderTabButton { text: "Projects", tab: 1 }
                HeaderTabButton { text: "Files", tab: 2 }
            }
        }
    }
}

#[component]
pub fn OpenTab(tab: usize) -> Element {
    let window = use_window();
    window.set_title("Marionette Analysis Selector");
    window.set_resizable(true);
    window.set_min_inner_size(Some(LogicalSize::new(450, 300)));

    rsx! {
        style { {include_str!("resources/styles/analysis-selector/analysis-selector.css")} }
        header_container {}
        {
            match tab {
                1 => rsx!( projects_container {} ),
                2 => rsx!( explorer_container {} ),
                _ => rsx! {
                    div {
                        class: "repeating_diagonal_lines"
                    }
                }
            }
        }
        bottom_bar_container {}
    }
}