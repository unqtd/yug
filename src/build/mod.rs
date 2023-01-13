use crate::{project_config::ProjectConfig, runnable::Runnable, util::handle_output};
use clap::Args;
use std::{error::Error, fs};

use self::buildsystem::{BuildOption, BuildSystem};

pub mod buildsystem;
pub mod compiler_interface;
pub mod objcopy_interface;

#[derive(Args, Debug)]
pub struct Build {
    /// Sets the clock frequency in megahertz
    #[arg(long)]
    mhz: Option<u8>,
    /// Displays all the commands used for the build
    #[arg(long)]
    watch: bool,
    /// Choosing the optimization level
    #[arg(long)]
    opt_level: Option<String>,
}

impl Runnable for Build {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let config = ProjectConfig::read_from_file("yug.toml")?;
        Ok(self.compile_project(&config))
    }
}

impl Build {
    fn compile_project(&self, config: &ProjectConfig) {
        let _ = fs::create_dir(&config.structure.builds);

        let mut build_system = BuildSystem::new(config);
        build_system
            .option_from(self.mhz.map(BuildOption::MHz))
            .option_from(self.opt_level.clone().map(BuildOption::OptLevel));

        // Compilation of project
        handle_output(self.watch, build_system.compile());

        // Proccessing by objcopy
        handle_output(self.watch, build_system.objcopy());

        println!("Compiled.")
    }
}
