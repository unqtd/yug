use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Dependence {
    Local {
        local: String,
        #[serde(default)]
        manifest: Manifest,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub sources: Vec<String>,
    pub headers: Vec<String>,
}

impl Default for Manifest {
    fn default() -> Self {
        Manifest {
            sources: vec![".".to_string()],
            headers: vec![".".to_string()],
        }
    }
}
