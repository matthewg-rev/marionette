// Purpose: A Plugin interface for the core library.
// Sources: https://adventures.michaelfbryan.com/posts/plugins-in-rust/
// Path: src\plugin.rs

use std::collections::HashMap;
use std::rc::Rc;
use libloading::Library;

#[derive(Debug)]
pub enum PluginError {
    NotImplemented(String),
    InvalidInput(String)
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PluginError::NotImplemented(message) => write!(f, "Not implemented: {}", message),
            PluginError::InvalidInput(message) => write!(f, "Invalid input: {}", message)
        }
    }
}

pub trait PluginRegistrar {
    fn register_function(&mut self, name: &str, function: Box<dyn Function>);
}

pub trait Function {
    // we are gonna have to pass a vector of bytes of a struct for the args and then
    // interpret the bytes as a struct when the plugin receives it.
    // the plugin then should return a Vec of bytes of a struct that we can interpret
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError>;
}

pub struct Plugin {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar)
}

pub struct FunctionProxy {
    function: Box<dyn Function>,
    library: Rc<Library>
}

impl Function for FunctionProxy {
    fn call(&self, args: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        self.function.call(args)
    }
}

pub struct PluginLibrary {
    pub functions: HashMap<String, FunctionProxy>,
    pub library: Rc<Library>
}

impl PluginLibrary {
    pub fn new(library: Rc<Library>) -> PluginLibrary {
        PluginLibrary {
            functions: HashMap::new(),
            library
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionProxy> {
        self.functions.get(name)
    }
}

impl PluginRegistrar for PluginLibrary {
    fn register_function(&mut self, name: &str, function: Box<dyn Function>) {
        self.functions.insert(name.to_string(), FunctionProxy {
            function,
            library: Rc::clone(&self.library)
        });
    }
}
