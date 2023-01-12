use std::fs;

use clap::Args;
use itertools::Itertools;

use crate::{
    build::compiletion_api::CompilerInterface,
    project_config::ProjectConfig,
    runnable::Runnable,
    util::{report_error, sh},
};

use self::dependence::Dependence;

pub mod dependence;

#[derive(Args, Debug)]
pub struct Deps {
    /// Choosing the optimization level
    #[arg(long)]
    opt_level: Option<String>,
}

impl Runnable for Deps {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ProjectConfig::read_from_file("yug.toml")?;

        let _ = fs::create_dir("vendor");

        for (name, dep) in &config.dependencies {
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

            report_error(sh(
                format!("cp {headers} vendor/{name}", headers = headers, name = name).as_str(),
                "Failed to copy headers",
            ));

            let mut compiler_api = CompilerInterface::from(&config);
            compiler_api.custom("-c");
            compiler_api.language(language);

            if let Some(level) = &self.opt_level {
                compiler_api.opt_level(level.as_str())
            }

            report_error(
                compiler_api
                    .gcc_avr(&sources, format!("vendor/{name}/obj/{name}.o").as_str())
                    .0,
            );
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
