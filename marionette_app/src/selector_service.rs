use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::prelude::*;
use dioxus_router::{use_route, use_router};
use dioxus_desktop::{LogicalSize, use_window};
use dioxus_html_macro::*;
use std::fs::*;
use std::os::windows::fs::MetadataExt;
use std::time::SystemTime;
use chrono::*;
use futures::StreamExt;

use crate::on_result;
use crate::msgbox::{Msg, MsgButtons, MsgResult, MsgSize, MsgType};

#[derive(Clone)]
pub struct FileEntry {
    pub file_name: String,
    pub file_size: String,
    pub file_path: String,
    pub file_extension: String,
    pub metadata: Metadata,
    pub date_modified: String,
}

#[derive(Clone)]
pub struct ExplorerState {
    // example: vec!["C:", "Users", "User", "Desktop", "Marionette", "Projects", "Project1", "Analysis1", "Analysis1.db"]
    pub path_stack: Vec<String>,

    // when we press the back button we need to store the path we are leaving
    // however once we go forward again anywhere, unless its the top of the stack, we need to clear the history
    pub path_history: Vec<String>,

    // example: vec!["something.txt", "something_else.txt"]
    pub current_files: Vec<FileEntry>,

    // example: vec!["folder1", "folder2"]
    pub current_folders: Vec<FileEntry>,
}

impl ExplorerState {
    pub fn new() -> Self {
        let mut self_ = Self {
            path_stack: vec![],
            current_files: vec![],
            current_folders: vec![],
            path_history: vec![],
        };

        let current_working_directory = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
        self_.set_working_directory(current_working_directory);
        self_.refresh_directory();
        self_
    }

    pub fn full_path(&self) -> String {
        let mut path = String::new();
        for (i, path_part) in self.path_stack.iter().enumerate() {
            path.push_str(path_part);
            if i != self.path_stack.len() - 1 {
                path.push('\\');
            }
        }
        path
    }

    pub fn refresh_directory(&mut self) {
        let path = self.full_path();
        let mut files = vec![];
        let mut folders = vec![];

        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();
            let file_path = entry.path().into_os_string().into_string().unwrap();
            let metadata = entry.metadata().unwrap();
            let date_modified = metadata.modified().unwrap();
            let date_modified = date_modified.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let date_modified = NaiveDateTime::from_timestamp_opt(date_modified as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();

            let file_extension = if metadata.is_dir() {
                String::from("Folder")
            } else {
                let mut file_extension = String::from("");
                if let Some(extension) = entry.path().extension() {
                    file_extension = extension.to_str().unwrap().to_string();
                }
                file_extension
            };

            let file_size = metadata.len();
            let file_size = if file_size < 1024 {
                format!("{} B", file_size)
            } else if file_size < 1024 * 1024 {
                format!("{:.2} KB", file_size as f64 / 1024.0)
            } else if file_size < 1024 * 1024 * 1024 {
                format!("{:.2} MB", file_size as f64 / 1024.0 / 1024.0)
            } else {
                format!("{:.2} GB", file_size as f64 / 1024.0 / 1024.0 / 1024.0)
            };

            // get file attributes
            let file_attributes = metadata.file_attributes();
            //let is_hidden = file_attributes & 2 != 0;
            //let is_readonly = file_attributes & 1 != 0;
            let is_system = file_attributes & 4 != 0;
            if is_system {
                continue;
            }

            if metadata.is_dir() {
                folders.push(FileEntry {
                    file_name,
                    file_path,
                    metadata,
                    date_modified,
                    file_size,
                    file_extension
                });
            } else {
                files.push(FileEntry {
                    file_name,
                    file_path,
                    metadata,
                    date_modified,
                    file_size,
                    file_extension
                });
            }
        }

        self.current_files = files;
        self.current_folders = folders;
    }

    pub fn go_up(&mut self) {
        if !self.path_stack.is_empty() {
            let mut path_part = self.path_stack.pop().unwrap();
            while path_part.is_empty() || path_part == "\\" {
                path_part = self.path_stack.pop().unwrap();
            }
            self.path_history.push(path_part);
            self.refresh_directory();
        }
    }

    pub fn go_back(&mut self) {
        if !self.path_history.is_empty() {
            self.path_stack.push(self.path_history.pop().unwrap());
            self.refresh_directory();
        }
    }

    pub fn go_into_dir(&mut self, dir_name: String) {
        self.path_stack.push(dir_name);
        self.refresh_directory();
    }

    pub fn set_working_directory(&mut self, path: String) {
        let mut folders = vec![];
        let mut current_folder = String::new();
        for c in path.chars() {
            if c == '\\' {
                folders.push(current_folder);
                current_folder = String::new();
            } else {
                current_folder.push(c);
            }
        }
        folders.push(current_folder);
        self.path_stack = folders;
        self.path_history = vec![];
    }
}

#[derive(Clone)]
pub struct SelectorState {
    pub selected_path: String,
    pub explorer_state: ExplorerState,
}

impl SelectorState {
    pub fn new() -> Self {
        Self {
            selected_path: String::new(),
            explorer_state: ExplorerState::new()
        }
    }
}

impl Debug for FileEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileEntry")
            .field("file_name", &self.file_name)
            .field("file_path", &self.file_path)
            .field("metadata", &self.metadata)
            .field("date_modified", &self.date_modified)
            .field("file_size", &self.file_size)
            .field("file_extension", &self.file_extension)
            .finish()
    }
}

impl Debug for ExplorerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExplorerState")
            .field("current_files", &self.current_files)
            .field("current_folders", &self.current_folders)
            .field("path_stack", &self.path_stack)
            .field("path_history", &self.path_history)
            .finish()
    }
}

impl Debug for SelectorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectorState")
            .field("selected_path", &self.selected_path)
            .field("explorer_state", &self.explorer_state)
            .finish()
    }
}

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
        style { include_str!("resources/styles/analysis-selector.css") }
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