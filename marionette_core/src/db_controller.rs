use std::collections::HashMap;
use serde_json::*;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum DatabaseType {
    DatabaseDirectory,
    UserSettings,
    ProjectSettings,
    AssembledData,
    AnalysisData
}

pub struct DatabaseController {
    pub directory: Database,
    pub databases: Vec<Database>,
    pub checker: CompatibilityChecker
}

pub struct CompatibilityChecker {
    pub compatibility_history: HashMap<DatabaseType, HashMap<String, Vec<String>>>,
    pub marionette_version: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompatibilityNoteLevel {
    Warning,
    Error
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompatibilityNote {
    pub key: String,
    pub message: String,
    pub level: CompatibilityNoteLevel
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompatibilityReport {
    pub database_type: DatabaseType,
    pub database_name: String,
    pub database_version: String,
    pub marionette_version: String,
    pub incompatible_fields: Vec<CompatibilityNote>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Database {
    pub database_type: DatabaseType,
    pub name: String,
    pub path: String,
    pub data: Value
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserSettings {
    pub database: Database
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProjectSettings {
    pub database: Database
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssembledData {
    pub database: Database
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnalysisData {
    pub database: Database
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DatabaseType::DatabaseDirectory => write!(f, "database_directory"),
            DatabaseType::UserSettings => write!(f, "user_settings"),
            DatabaseType::ProjectSettings => write!(f, "project_settings"),
            DatabaseType::AssembledData => write!(f, "assembled_data"),
            DatabaseType::AnalysisData => write!(f, "analysis_data")
        }
    }
}

impl std::fmt::Display for CompatibilityNoteLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompatibilityNoteLevel::Warning => write!(f, "warning"),
            CompatibilityNoteLevel::Error => write!(f, "error")
        }
    }
}

impl std::fmt::Display for CompatibilityNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: [{}] {}", self.level, self.key, self.message)
    }
}

impl std::fmt::Display for CompatibilityReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "🚀 Compatibility report for {} ({}, {}):", self.database_name, self.database_type, self.database_version)?;
        if self.incompatible_fields.is_empty() {
            writeln!(f, "\t- ✅ No incompatibilities")?;
        } else {
            for note in &self.incompatible_fields {
                if note.level == CompatibilityNoteLevel::Warning {
                    writeln!(f, "\t- ⚠️ {}", note)?;
                } else {
                    writeln!(f, "\t- ❌ {}", note)?;
                }
            }
        }
        Ok(())
    }
}

impl From<String> for DatabaseType {
    fn from(string: String) -> Self {
        match string.as_str() {
            "database_directory" => DatabaseType::DatabaseDirectory,
            "user_settings" => DatabaseType::UserSettings,
            "project_settings" => DatabaseType::ProjectSettings,
            "assembled_data" => DatabaseType::AssembledData,
            "analysis_data" => DatabaseType::AnalysisData,
            _ => panic!("invalid database type")
        }
    }
}

impl Default for DatabaseController {
    fn default() -> Self { Self::new() }
}

impl std::ops::Index<&str> for Database {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<&str> for Database {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl std::ops::Index<&str> for UserSettings {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.database[index]
    }
}

impl std::ops::Index<&str> for ProjectSettings {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.database[index]
    }
}

impl std::ops::Index<&str> for AssembledData {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.database[index]
    }
}

impl std::ops::Index<&str> for AnalysisData {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.database[index]
    }
}

impl From<Database> for UserSettings {
    fn from(database: Database) -> Self {
        if database.database_type != DatabaseType::UserSettings {
            panic!("database is not of type UserSettings");
        }
        UserSettings {
            database
        }
    }
}

impl From<Database> for ProjectSettings {
    fn from(database: Database) -> Self {
        if database.database_type != DatabaseType::ProjectSettings {
            panic!("database is not of type ProjectSettings");
        }
        ProjectSettings {
            database
        }
    }
}

impl From<Database> for AssembledData {
    fn from(database: Database) -> Self {
        if database.database_type != DatabaseType::AssembledData {
            panic!("database is not of type AssembledData");
        }
        AssembledData {
            database
        }
    }
}

impl From<Database> for AnalysisData {
    fn from(database: Database) -> Self {
        if database.database_type != DatabaseType::AnalysisData {
            panic!("database is not of type AnalysisData");
        }
        AnalysisData {
            database
        }
    }
}

impl Drop for DatabaseController {
    fn drop(&mut self) {
        self.save_directory();
        self.save_databases();
    }
}

impl DatabaseController {
    pub fn new() -> DatabaseController {
        let mut self_ = DatabaseController {
            directory: DatabaseController::read_directory(),
            databases: Vec::new(),
            checker: CompatibilityChecker::default()
        };

        self_.read_all_databases();
        self_
    }

    pub fn project_exists(&self, project_name: &str) -> bool {
        for database in &self.databases {
            if database.database_type == DatabaseType::ProjectSettings && database["project_name"].as_str().unwrap() == project_name {
                return true;
            }
        }
        false
    }

    pub fn new_project(&mut self, project_name: &str, project_file: &str) {
        let project_value = json!({
            "marionette_version": env!("CARGO_PKG_VERSION"),

            "project_name": project_name,
            "project_files": [project_file]
        });
        // we want to put this json file in project_file's directory
        let project_path = std::path::Path::new(project_file);
        let project_directory = project_path.parent().unwrap().to_str().unwrap().to_string();
        std::fs::write(format!("{}/{}.json", project_directory, project_name), serde_json::to_string_pretty(&project_value).unwrap()).unwrap();

        self.directory["databases"].as_array_mut().unwrap().push(json!({
            "database_type": "project_settings",
            "database_name": project_name,
            "database_path": format!("{}/{}.json", project_directory, project_name)
        }));
    }

    pub fn save_directory(&self) {
        std::fs::write("directory.json", serde_json::to_string_pretty(&self.directory.data).unwrap()).unwrap();
    }

    pub fn save_databases(&self) {
        for database in &self.databases {
            std::fs::write(database.path.as_str(), serde_json::to_string_pretty(&database.data).unwrap()).unwrap();
        }
    }

    pub fn read_all_databases(&mut self) {
        for database_info in self.directory["databases"].as_array().unwrap() {
            let database_type = database_info["database_type"].as_str().unwrap();
            let database_path = database_info["database_path"].as_str().unwrap();
            let mut database = DatabaseController::read_database(database_type, database_path);
            /* compatibility check */
            let report = self.checker.check_compatibility(database.clone());
            if !report.incompatible_fields.is_empty() {
                println!("incompatible fields found in database: {}", database_path);
                for note in report.incompatible_fields {
                    println!("{}: {}", note.key, note.message);
                }
            }
            database.name = database_info["database_name"].as_str().unwrap().to_string();
            self.databases.push(database);
        }
    }

    pub fn read_database(database_type: &str, database_path: &str) -> Database {
        let data = std::fs::read_to_string(database_path).expect("failed to read database");
        let database_value: Value = serde_json::from_str(data.as_str()).unwrap();
        Database {
            database_type: DatabaseType::from(database_type.to_string()),
            data: database_value,
            path: database_path.to_string(),
            name: "".to_string()
        }
    }

    pub fn read_directory() -> Database {
        let data = std::fs::read_to_string("directory.json").expect("failed to read directory");
        let directory_value = serde_json::from_str(data.as_str());
        Database {
            database_type: DatabaseType::DatabaseDirectory,
            data: directory_value.unwrap(),
            path: "directory.json".to_string(),
            name: "directory".to_string()
        }
    }

    pub fn get_databases_of_type(&self, database_type: DatabaseType) -> Vec<&Database> {
        let mut databases = Vec::new();
        for database in &self.databases {
            if database.database_type == database_type {
                databases.push(database);
            }
        }
        databases
    }

    pub fn get_project_databases(&self) -> Vec<ProjectSettings> {
        let mut databases = Vec::new();
        for database in &self.databases {
            if database.database_type == DatabaseType::ProjectSettings {
                databases.push(ProjectSettings::from(database.clone()));
            }
        }
        databases
    }

    pub fn get_user_databases(&self) -> Vec<UserSettings> {
        let mut databases = Vec::new();
        for database in &self.databases {
            if database.database_type == DatabaseType::UserSettings {
                databases.push(UserSettings::from(database.clone()));
            }
        }
        databases
    }

    pub fn get_assembled_databases(&self) -> Vec<AssembledData> {
        let mut databases = Vec::new();
        for database in &self.databases {
            if database.database_type == DatabaseType::AssembledData {
                databases.push(AssembledData::from(database.clone()));
            }
        }
        databases
    }

    pub fn get_analysis_databases(&self) -> Vec<AnalysisData> {
        let mut databases = Vec::new();
        for database in &self.databases {
            if database.database_type == DatabaseType::AnalysisData {
                databases.push(AnalysisData::from(database.clone()));
            }
        }
        databases
    }
}

impl Default for CompatibilityChecker {
    fn default() -> Self {
        let mut checker = Self::new();

        let database_directory = checker.compatibility_history.get_mut(&DatabaseType::DatabaseDirectory).unwrap();
        database_directory.insert("0.1.0".to_string(), vec![
            "marionette_version".to_string(), // this will literally never change

            "databases".to_string(),
            "database_type".to_string(),
            "database_name".to_string(),
            "database_path".to_string(),
        ]);

        let user_settings = checker.compatibility_history.get_mut(&DatabaseType::UserSettings).unwrap();
        user_settings.insert("0.1.0".to_string(), vec![
            "marionette_version".to_string(), // this will literally never change

            "operation_code_casing".to_string(),
            // TODO: add more user settings
        ]);

        let project_settings = checker.compatibility_history.get_mut(&DatabaseType::ProjectSettings).unwrap();
        project_settings.insert("0.1.0".to_string(), vec![
            "marionette_version".to_string(), // this will literally never change

            "project_name".to_string(),
            "project_files".to_string(),
            "analysis_db_path".to_string(),
            "assembled_db_path".to_string(),
        ]);

        let assembled_data = checker.compatibility_history.get_mut(&DatabaseType::AssembledData).unwrap();
        assembled_data.insert("0.1.0".to_string(), vec![
            "marionette_version".to_string(), // this will literally never change

            // TODO: assembler hasn't been made yet, but once it is, must revisit
        ]);

        let analysis_data = checker.compatibility_history.get_mut(&DatabaseType::AnalysisData).unwrap();
        analysis_data.insert("0.1.0".to_string(), vec![
            "marionette_version".to_string(), // this will literally never change

            // TODO: analyzer hasn't been made yet, but once it is, must revisit
        ]);

        checker
    }
}

impl CompatibilityChecker {
    pub fn new() -> CompatibilityChecker {
        let mut self_ = CompatibilityChecker {
            compatibility_history: HashMap::new(),
            marionette_version: env!("CARGO_PKG_VERSION").to_string()
        };

        self_.compatibility_history.insert(DatabaseType::DatabaseDirectory, HashMap::new());
        self_.compatibility_history.insert(DatabaseType::UserSettings, HashMap::new());
        self_.compatibility_history.insert(DatabaseType::ProjectSettings, HashMap::new());
        self_.compatibility_history.insert(DatabaseType::AssembledData, HashMap::new());
        self_.compatibility_history.insert(DatabaseType::AnalysisData, HashMap::new());

        self_
    }

    pub fn check_compatibility(&self, database: Database) -> CompatibilityReport {
        let db_type = &database.database_type;
        let db_version_str = database["marionette_version"].as_str().unwrap();
        let db_type = self.compatibility_history.get(db_type).unwrap();
        let db_version = db_type.get(db_version_str);

        let marionette_version = db_type.get(&self.marionette_version).unwrap();

        let mut report = CompatibilityReport {
            database_type: database.database_type.clone(),
            database_version: db_version_str.to_string(),
            database_name: database.name.clone(),
            marionette_version: self.marionette_version.clone(),
            incompatible_fields: Vec::new()
        };
        if let Some(db_version) = db_version {
            for key in db_version {
                if !marionette_version.contains(key) {
                    report.incompatible_fields.push(CompatibilityNote {
                        key: key.clone(),
                        message: format!("this key is not used but does not exist in marionette version {}", self.marionette_version).to_string(),
                        level: CompatibilityNoteLevel::Warning
                    });
                }
            }

            let mut used_keys = Vec::new();
            fn collect_keys(value: &Value, used_keys: &mut Vec<String>) {
                match value {
                    Value::Object(map) => {
                        for (key, value) in map {
                            used_keys.push(key.to_string());
                            collect_keys(value, used_keys);
                        }
                    },
                    Value::Array(array) => {
                        for value in array {
                            collect_keys(value, used_keys);
                        }
                    },
                    _ => {}
                }
            }
            collect_keys(&database.data, &mut used_keys);

            for key in used_keys {
                if !marionette_version.contains(&key) {
                    let mut already_warned = false;
                    for note in &report.incompatible_fields {
                        if note.key == key {
                            already_warned = true;
                            break;
                        }
                    }

                    if already_warned {
                        report.incompatible_fields.retain(|note| note.key != key);
                    }

                    report.incompatible_fields.push(CompatibilityNote {
                        key: key.clone(),
                        message: format!("this key is used but is not available in marionette version {}", self.marionette_version).to_string(),
                        level: CompatibilityNoteLevel::Error
                    });
                }
            }
        } else {
            report.incompatible_fields.push(CompatibilityNote {
                key: db_version_str.to_string(),
                message: "project marionette version doesn't exist within the compatibility checker.".to_string(),
                level: CompatibilityNoteLevel::Error
            });
        }

        report
    }
}

impl CompatibilityReport {
    pub fn has_warnings(&self) -> bool {
        for note in &self.incompatible_fields {
            if note.level == CompatibilityNoteLevel::Warning {
                return true;
            }
        }
        false
    }

    pub fn has_errors(&self) -> bool {
        for note in &self.incompatible_fields {
            if note.level == CompatibilityNoteLevel::Error {
                return true;
            }
        }
        false
    }

    pub fn compatible(&self) -> bool {
        !self.has_warnings() && !self.has_errors()
    }
}