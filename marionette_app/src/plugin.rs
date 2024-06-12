use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use serde_json::Value;
use std::{os, fs, path};

#[derive(Clone)]
pub struct PluginSetting {
    pub setting: String,

    pub flag: Option<bool>,
    pub string: Option<String>,
    pub number: Option<f64>
}

#[derive(Clone)]
pub struct Plugin {
    // metadata
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub enabled: bool,
    
    // user data
    pub settings: Vec<PluginSetting>,

    // plugin instance
    pub code: String,
}

#[pyclass]
struct PluginInterface {
    plugin: Plugin
}

#[pymethods]
impl PluginInterface {
    fn get_str_setting(&self, setting: &str) -> Option<String> {
        for plugin_setting in &self.plugin.settings {
            if plugin_setting.setting == setting {
                return plugin_setting.string.clone();
            }
        }
        None
    }

    fn get_num_setting(&self, setting: &str) -> Option<f64> {
        for plugin_setting in &self.plugin.settings {
            if plugin_setting.setting == setting {
                return plugin_setting.number.clone();
            }
        }
        None
    }

    fn get_bool_setting(&self, setting: &str) -> Option<bool> {
        for plugin_setting in &self.plugin.settings {
            if plugin_setting.setting == setting {
                return plugin_setting.flag.clone();
            }
        }
        None
    }
}

impl Plugin {
    pub fn get_bool(value: Value, key: &str) -> Result<bool, ()> {
        if value[key].is_boolean() {
            Ok(value[key].as_bool().unwrap())
        } else {
            Err(())
        }
    }

    pub fn get_str(value: Value, key: &str) -> Result<String, ()> {
        if value[key].is_string() {
            Ok(value[key].as_str().unwrap().to_string())
        } else {
            Err(())
        }
    }
    
    pub fn get_num(value: Value, key: &str) -> Result<f64, ()> {
        if value[key].is_number() {
            Ok(value[key].as_f64().unwrap())
        } else {
            Err(())
        }
    }

    pub fn new(file_path: &str) -> Result<Plugin, ()> {
        let path = path::Path::new(file_path);
        if !path.exists() || !path.is_file() || path.extension().unwrap_or_default() != "py" {
            return Err(());
        }

        let file = fs::read_to_string(file_path).unwrap();
        let mut plugin = Plugin {
            name: String::new(),
            version: String::new(),
            author: String::new(),
            description: String::new(),
            enabled: false,

            settings: vec![],

            code: file,
        };

        // get non-file-extension name
        let name = path.file_stem();
        if name.is_none() {
            panic!("File name is empty: {}", file_path);
        }
        let name = name.unwrap().to_str().unwrap();

        // check if json config exists
        let json_path = path.parent().unwrap().join(format!("{}.json", name));
        
        if json_path.exists() {
            let json = fs::read_to_string(json_path.clone()).unwrap();
            let json: serde_json::Value = serde_json::from_str(&json).unwrap();

            let name = Plugin::get_str(json.clone(), "name");
            let version = Plugin::get_str(json.clone(), "version");
            let author = Plugin::get_str(json.clone(), "author");
            let description = Plugin::get_str(json.clone(), "description");
            let enabled = Plugin::get_bool(json.clone(), "enabled");

            match (name, version, author, description, enabled) {
                (Ok(name), Ok(version), Ok(author), Ok(description), Ok(enabled)) => {
                    plugin.name = name;
                    plugin.version = version;
                    plugin.author = author;
                    plugin.description = description;
                    plugin.enabled = enabled;
                },
                _ => panic!("JSON config for py-plugin is invalid: {}", json_path.to_str().unwrap())
            }

            if json["settings"].is_object() {
                for (setting, value) in json["settings"].as_object().unwrap() {
                    if value.is_object() {
                        let mut plugin_setting = PluginSetting {
                            setting: String::new(),
                            flag: None,
                            string: None,
                            number: None
                        };
                        plugin_setting.setting = setting.clone();

                        let flag = Plugin::get_bool(value.clone(), "flag");
                        let string = Plugin::get_str(value.clone(), "string");
                        let number = Plugin::get_num(value.clone(), "number");

                        match (flag, string, number) {
                            (Ok(flag), _, _) => plugin_setting.flag = Some(flag),
                            (_, Ok(string), _) => plugin_setting.string = Some(string),
                            (_, _, Ok(number)) => plugin_setting.number = Some(number),
                            _ => panic!("JSON config for py-plugin is invalid: {}", json_path.to_str().unwrap())
                        }

                        plugin.settings.push(plugin_setting);
                    }
                }
            }
        } else {
            panic!("JSON config for py-plugin does not exist: {}", json_path.to_str().unwrap());
        }
        
        Ok(plugin)
    }

    pub fn init(&self) -> PyResult<()> {
        Python::with_gil(|py| {
            println!("creating python context for plugin: {}", self.name);
            let plugin_interface = PluginInterface { plugin: self.clone() };
            let plugin_interface = Py::new(py, plugin_interface).unwrap();
            
            let globals = [("plugin_interface", plugin_interface)].into_py_dict_bound(py);
            let res = py.run_bound(self.code.as_str(), Some(&globals), None);
            match res {
                Ok(_) => println!("plugin {} initialized", self.name),
                Err(e) => println!("plugin {} failed to initialize: {}", self.name, e)
            }

            Ok(())
        })
    }
}