use std::collections::HashMap;

use bincode::de::{Decoder};
use bincode::de::read::Reader;
use bincode::config::{Config};
use bincode::{Decode};
use bincode::error::{DecodeError};
use crate::mproj::{MarionetteProject, RawProject};

pub struct ProjectUpgrader {
    upgrade_functions: HashMap<(String, String), Box<dyn Fn(RawProject) -> MarionetteProject>>
}

impl ProjectUpgrader {
    pub fn new() -> ProjectUpgrader {
        let mut upgrade_functions: HashMap<(String, String), Box<dyn Fn(RawProject) -> MarionetteProject>> = HashMap::new();
        upgrade_functions.insert(("0.0.1".to_string(), "0.1.0".to_string()), Box::new(|project| {
            let mut new_project = MarionetteProject::new();

            

            new_project
        }));

        ProjectUpgrader {
            upgrade_functions,
        }
    }
}