use std::{collections::HashMap, error::Error, fs};

mod clangd;
mod gitinit;
mod mainc;
mod spec;
mod util;

use clap::Args;

use crate::{
    project_config::{Compiler, Firmware, Language, ProjectConfig, Structure},
    runnable::Runnable,
};

use self::util::{get_name_current_dir, write_file};

#[derive(Args, Debug)]
pub struct Init {
    /// The name of the project under which the directory with this name will be created
    project_name: Option<String>,
    /// Microcontroller model
    #[arg(short, long)]
    target: String,
    /// Sets C++ as the project language
    #[arg(long)]
    cpp: bool,
    /// Initializes the local git repository
    #[arg(long)]
    git: bool,
    /// Organizes a complete development environment
    #[arg(short, long)]
    dev: bool,
    /// Clock frequency in megahertz. By default it is 1mhz
    #[arg(long)]
    mhz: Option<u8>,
}

impl Runnable for Init {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let config = self.to_config();
        let directory = self
            .project_name
            .as_ref()
            .map(|name| {
                let _ = fs::create_dir(&name);
                name.to_string()
            })
            .unwrap_or(".".to_string());

        // Create sources directory
        let _ = fs::create_dir(format!("{}/{}", &directory, config.structure.sources));

        // Create main.c file
        self.create_main_c(&directory, &config);

        // Create config file
        write_file(
            format!("{}/yug.toml", &directory),
            toml::to_string(&config).unwrap().trim_end(),
        );

        self.git_init_repo(&directory);

        // Create a include directory for headers
        if self.dev {
            self.create_clangd(&directory);

            let include_path = format!("{}/{}", &directory, &config.structure.includes);
            let _ = fs::create_dir(&include_path);

            self.create_spec_filel(&include_path)
        }

        Ok(println!("Done. 🐪"))
    }
}

impl Init {
    fn to_config(&self) -> ProjectConfig {
        ProjectConfig {
            firmware: Firmware {
                name: (self
                    .project_name
                    .to_owned()
                    .unwrap_or_else(get_name_current_dir)),
                target: self.target.to_owned(),
                language: (if self.cpp { Language::Cpp } else { Language::C }),
            },
            structure: Structure::default(),
            compiler: Compiler::default(),
            dependencies: HashMap::new(),
        }
    }
}
