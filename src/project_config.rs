use std::{collections::HashMap, error::Error, fs, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub firmware: Firmware,

    #[serde(default)]
    pub structure: Structure,

    #[serde(default)]
    pub compiler: Compiler,

    #[serde(skip_serializing)]
    #[serde(default)]
    pub utils: Utils,

    #[serde(skip_serializing)]
    #[serde(default)]
    pub externlibs: HashMap<String, ExternLib>,
}

/////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Language {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Firmware {
    pub name: String,
    pub target: Target,
    #[serde(skip_serializing_if = "Language::is_c")]
    #[serde(default)]
    pub language: Language,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub model: String,
    pub mhz: u8,
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
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Utils {
    #[serde(flatten)]
    pub utils: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExternLib {
    pub objs: Vec<String>,
}

/////////////////////////////////////////////////////

impl Language {
    const fn is_c(&self) -> bool {
        match self {
            Self::C => true,
            Self::Cpp => false,
        }
    }

    pub const fn compiler(&self) -> &'static str {
        match self {
            Self::C => "avr-gcc",
            Self::Cpp => "avr-g++",
        }
    }

    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::C => "c",
            Self::Cpp => "cpp",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::C
    }
}

/////////////////////////////////////////////////////

impl Default for Structure {
    fn default() -> Self {
        Self {
            sources: "src".to_string(),
            builds: "_build".to_string(),
            includes: "include".to_string(),
        }
    }
}

/////////////////////////////////////////////////////

impl ProjectConfig {
    pub fn read_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        if let Ok(mut file) = fs::File::open(path) {
            let mut text_config_file = String::new();
            file.read_to_string(&mut text_config_file)?;

            toml::from_str(&text_config_file).map_err(|err| format!("TomlParser: {err}").into())
        } else {
            Err("Not found yug.toml in current directory...".into())
        }
    }
}
