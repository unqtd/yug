use crate::{
    project_config::ProjectConfig,
    runnable::Runnable,
    util::{get_line_of_all_namefiles_in_dir_with_ext, report_error},
};
use clap::Args;
use std::{error::Error, fs, process::Output};

pub mod compiletion_api;
use compiletion_api::CompilerInterface;

#[derive(Args, Debug)]
pub struct Build {
    #[arg(long)]
    mhz: Option<u8>,
    // #[arg(long)]
    // release: bool,
    /// Outputs all used commands
    #[arg(long)]
    watch: bool,
    // Level of optimization
    // #[arg(long)]
    // opt_level: Option<String>,
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

        let mut compiler_api = CompilerInterface::from(config);

        if let Some(mhz) = self.mhz {
            compiler_api.mhz(mhz);
        }

        let sources = get_line_of_all_namefiles_in_dir_with_ext(
            config.structure.sources.as_str(),
            config.firmware.language.to_str(),
        );

        let objects = get_line_of_all_namefiles_in_dir_with_ext("vendor", "o");
        let sources_and_objects = sources + " " + &objects;

        let builds = format!("{}/firmware.elf", config.structure.builds);

        self.handle_output(compiler_api.gcc_avr(&sources_and_objects, &builds));
        self.handle_output(compiler_api.obj_copy());

        println!("Compiled.")
    }

    fn handle_output(&self, (output, cmd): (Output, String)) {
        if self.watch {
            println!("{}", cmd.trim())
        }

        report_error(output)
    }
}
