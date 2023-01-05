use crate::{
    project_config::{Language, ProjectConfig},
    runnable::Runnable,
    util::{get_line_of_all_namefiles_in_dir_with_ext, report_error, sh},
};
use clap::Args;
use std::{error::Error, fs, process::Output};

#[derive(Args, Debug)]
pub struct Build {
    #[arg(long)]
    mhz: Option<u8>,
    #[arg(long)]
    release: bool,
    /// Outputs all used commands
    #[arg(long)]
    watch: bool,
    // Level of optimization
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

        report_error(self.gcc_avr(&config));
        report_error(self.obj_copy(&config));

        println!("Compiled.")
    }

    fn gcc_avr(&self, config: &ProjectConfig) -> Output {
        let mut arguments = vec![];

        if let Some(fcpu) = &self.mhz {
            arguments.push(format!("-DF_CPU={}000000UL", fcpu))
        }

        let avr_gcc_cmd = &self.format_avr_gcc_cmd(config, &arguments);

        if self.watch {
            println!("{}", &avr_gcc_cmd);
        }

        sh(avr_gcc_cmd, "Failed to execute avr-gcc/avr-g++ command")
    }

    fn obj_copy(&self, config: &ProjectConfig) -> Output {
        let obj_copy_cmd = self.format_avr_objcopy_cmd(config);

        if self.watch {
            println!("{}", obj_copy_cmd)
        }

        sh(&obj_copy_cmd, "Failed to execute avr-objcopy command")
    }

    fn format_avr_gcc_cmd(&self, config: &ProjectConfig, arguments: &Vec<String>) -> String {
        format!(
            "{cc} -Wall {optimization} {custom} -I{headers} -mmcu={arch} -o {builds}/firmware.elf {sources} {yet_sources}",
            cc = config.firmware.language.compiler(),
            optimization = self.level_of_optimization(&config) ,
            custom = format!("{} {}", config.compiler.custom, arguments.join(" ")),
            headers = config.structure.includes,
            arch = config.firmware.target.to_lowercase(),
            builds = config.structure.builds,
            sources = get_line_of_all_namefiles_in_dir_with_ext(&config.structure.sources, config.firmware.language.to_str()),
            yet_sources = match config.firmware.language {
                Language::C => String::new(),
                Language::Cpp => get_line_of_all_namefiles_in_dir_with_ext(&config.structure.sources, "c")
            }
        )
    }

    fn format_avr_objcopy_cmd(&self, config: &ProjectConfig) -> String {
        format!(
            "avr-objcopy -j .text -j .data -O ihex {builds}/firmware.elf {builds}/firmware.hex",
            builds = config.structure.builds
        )
    }

    fn level_of_optimization(&self, config: &ProjectConfig) -> String {
        format!(
            "-O{}",
            if self.release {
                "3".to_string()
            } else {
                self.opt_level
                    .as_ref()
                    .or(config.compiler.opt_level.as_ref())
                    .map(String::from)
                    .unwrap_or("s".to_string())
            }
        )
    }
}
