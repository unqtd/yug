use std::{collections::HashMap, error::Error, fs, io::Read};

use serde::{Deserialize, Serialize};

use crate::deps::dependence::Dependence;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub firmware: Firmware,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub structure: Structure,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub compiler: Compiler,
    #[serde(skip_serializing)]
    pub dependencies: HashMap<String, Dependence>,
}

/////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Firmware {
    pub name: String,
    pub target: String,
    #[serde(skip_serializing_if = "Language::is_c")]
    #[serde(default)]
    pub language: Language,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Structure {
    pub sources: String,
    pub builds: String,
    pub includes: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Compiler {
    #[serde(default)]
    pub custom: String,
    // #[serde(rename = "opt-level")]
    // pub opt_level: Option<String>,
}

/////////////////////////////////////////////////////

impl Language {
    fn is_c(&self) -> bool {
        match self {
            Language::C => true,
            Language::Cpp => false,
        }
    }

    pub fn compiler(&self) -> &'static str {
        match self {
            Language::C => "avr-gcc",
            Language::Cpp => "avr-g++",
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Language::C => "c",
            Language::Cpp => "cpp",
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::C
    }
}

/////////////////////////////////////////////////////

impl Default for Structure {
    fn default() -> Self {
        Structure {
            sources: "src".to_string(),
            builds: "_build".to_string(),
            includes: "include".to_string(),
        }
    }
}

/////////////////////////////////////////////////////

impl ProjectConfig {
    pub fn read_from_file(path: &str) -> Result<ProjectConfig, Box<dyn Error>> {
        if let Ok(mut file) = fs::File::open(path) {
            let mut text_config_file = String::new();
            file.read_to_string(&mut text_config_file)?;

            toml::from_str::<ProjectConfig>(&text_config_file)
                .map_err(|err| format!("TomlParser: {}", err).into())
        } else {
            Err("Not found yug.toml in current directory...".into())
        }
    }
}
