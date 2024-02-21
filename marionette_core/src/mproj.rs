use std::fmt::{Debug};

pub mod upgrader;
pub mod reader;
pub mod writer;

pub struct RawProject {
    // Metadata
    pub project_version: String,

    pub raw_project: Vec<u8>,
}

#[derive(Debug)]
pub struct MarionetteProject {
    // Metadata
    pub project_version: String,

    pub project_name: String,
    pub project_files: Vec<String>,
}

impl MarionetteProject {
    pub fn new() -> MarionetteProject {
        MarionetteProject {
            project_version: env!("CARGO_PKG_VERSION").to_string(),
            project_name: "New Project".to_string(),
            project_files: Vec::new(),
        }
    }
}