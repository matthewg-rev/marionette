use std::collections::HashMap;
use crate::{byte_stream::{ByteStream, ByteStreamRead}, mproj::{MarionetteProject, RawProject}};

pub struct ProjectUpgrader {
    upgrade_functions: HashMap<(String, String), Box<dyn Fn(RawProject) -> MarionetteProject>>
}

impl ProjectUpgrader {
    pub fn new() -> ProjectUpgrader {
        let mut upgrade_functions: HashMap<(String, String), Box<dyn Fn(RawProject) -> MarionetteProject>> = HashMap::new();
        
        upgrade_functions.insert((env!("CARGO_PKG_VERSION").to_string(), env!("CARGO_PKG_VERSION").to_string()), Box::new(|proj| {
            let mut stream = ByteStream::new(proj.raw);
            MarionetteProject::read(&mut stream).unwrap()
        }));

        ProjectUpgrader {
            upgrade_functions,
        }
    }
}