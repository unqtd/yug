use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

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
    #[serde(default)]
    pub utils: Utils,

    #[serde(skip_serializing)]
    #[serde(default)]
    pub externlibs: HashMap<String, ExternLib>,
}

/////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct Firmware {
    pub name: String,
    pub language: Language,
    pub target: Target,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub model: String,
    pub mhz: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Language {
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
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
    pub fn read_from_file(path: &str) -> Result<Self, String> {
        let text_config = fs::read_to_string(path)
            .map_err(|_| "???? ???????????? ???????????? ???????? ??yug.toml?? ?? ?????????????? ????????????????????...")?;

        toml::from_str(&text_config).map_err(|err| format!("TomlParser: {err}"))
    }
}
