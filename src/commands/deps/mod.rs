use std::fs;

use clap::Args;
use itertools::Itertools;

use crate::{
    commands::build::{
        buildsystem::{BuildOption, BuildSystem},
        compiler_interface::CompilerOptions,
    },
    project_config::ProjectConfig,
    runnable::Runnable,
    util::{execute_command, handle_output, report_error, ExecutionMode},
};

use self::dependence::Dependence;

pub mod dependence;

#[derive(Args, Debug)]
pub struct Deps {
    /// Choosing the optimization level
    #[arg(long)]
    opt_level: Option<String>,
    /// Displays all the commands used for the build
    #[arg(long)]
    watch: bool,
}

impl Runnable for Deps {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        use BuildOption::*;

        let config = ProjectConfig::read_from_file("yug.toml")?;

        let _ = fs::create_dir("vendor");

        for (name, dep) in config.dependencies.iter() {
            let _ = fs::create_dir(format!("vendor/{}", name));
            let _ = fs::create_dir(format!("vendor/{name}/obj", name = name));

            let Dependence::Local {
                local,
                language,
                manifest,
            } = dep;

            let sources = Self::full_path(&local, &manifest.sources);
            let headers = Self::full_path(&local, &manifest.headers);

            // Debug
            println!("{}", sources);
            println!("{}", headers);

            // Just plug!
            // Not to use!

            // Copying
            report_error(execute_command(
                format!("cp {headers} vendor/{name}", headers = headers, name = name).as_str(),
                "Failed to copy headers",
                ExecutionMode::Output,
            ));

            // Compilation of dependence

            let mut build_system = BuildSystem::new(&config);
            build_system
                .option(Custom("-c"))
                .option_from(self.opt_level.as_ref().map(OptLevel));

            let mut compiler_interface = build_system.get_compiler();
            compiler_interface
                .option(CompilerOptions::Languge(language))
                .output(format!("vendor/{name}/obj/{name}.o"));

            handle_output(self.watch, compiler_interface.compile());
        }

        Ok(())
    }
}

impl Deps {
    fn full_path(local: &str, patterns: &Vec<String>) -> String {
        patterns
            .iter()
            .map(|pat| format!("{local}/{pat}", local = local, pat = pat))
            .join(" ")
    }
}
