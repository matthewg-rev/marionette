// Purpose: A Plugin interface for the core library.
// Sources: https://adventures.michaelfbryan.com/posts/plugins-in-rust/
// Path: src\plugin.rs

pub mod interface;
pub mod comm;

use std::io;
use std::ffi::OsStr;
use std::rc::Rc;
use std::collections::HashMap;
use libloading::Library;

use interface::{Plugin, Function, PluginLibrary};

pub static PLUGIN_CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static PLUGIN_RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[derive(Default)]
pub struct PluginDirectory {
    registered_plugins: HashMap<String, PluginLibrary>
}

impl PluginDirectory {
    pub fn new() -> PluginDirectory {
        PluginDirectory::default()
    }

    pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> io::Result<()> {
        let name = library_path.as_ref().to_str().unwrap().split('\\').last().unwrap().to_string();
        let library = Library::new(library_path);
        if let Ok(library) = library {
            let library = Rc::new(library);
            let declaration = library.get::<*mut Plugin>(b"plugin_declaration\0").unwrap().read();
            if declaration.rustc_version != PLUGIN_RUSTC_VERSION {
                return Err(io::Error::new(io::ErrorKind::Other, "Plugin rustc version mismatch"));
            }
            if declaration.core_version != PLUGIN_CORE_VERSION {
                return Err(io::Error::new(io::ErrorKind::Other, "Plugin core version mismatch"));
            }

            let mut registrar = PluginLibrary::new(Rc::clone(&library));
            (declaration.register)(&mut registrar);

            self.registered_plugins.insert(name, registrar);

            return Ok(())
        }
        println!("{}", library.err().unwrap());
        Err(io::Error::new(io::ErrorKind::Other, "Failed to load library"))
    }

    pub fn get_supported_plugin(&self, bytes: &[u8]) -> Option<&PluginLibrary> {
        for (_, plugin) in self.registered_plugins.iter() {
            if let Some(function) = plugin.functions.get("can_disassemble") {
                let result = function.call(bytes.to_vec());
                if let Ok(result) = result {
                    if result.len() == 1 && result[0] == 1 {
                        return Some(plugin);
                    }
                }
            }
        }
        None
    }

    pub fn can_disassemble(&self, bytes: &[u8]) -> bool {
        for (_, plugin) in self.registered_plugins.iter() {
            if let Some(function) = plugin.functions.get("can_disassemble") {
                let result = function.call(bytes.to_vec());
                if let Ok(result) = result {
                    if result.len() == 1 {
                        return result[0] == 1;
                    }
                }
            }
        }
        false
    }
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[no_mangle]
        pub static plugin_declaration: $crate::plugin::interface::Plugin = $crate::plugin::interface::Plugin {
            rustc_version: $crate::plugin::PLUGIN_RUSTC_VERSION,
            core_version: $crate::plugin::PLUGIN_CORE_VERSION,
            register: $register
        };
    }
}