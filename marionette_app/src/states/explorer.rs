use std::fs::*;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use std::time::SystemTime;
use chrono::*;
use std::fmt::{Debug, Formatter};

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
            let date_modified = DateTime::from_timestamp(date_modified as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();

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
            if cfg!(windows) {
                //let is_hidden = file_attributes & 2 != 0;
                //let is_readonly = file_attributes & 1 != 0;
                let is_system = metadata.file_attributes() & 4 != 0;
                if is_system {
                    continue;
                }
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