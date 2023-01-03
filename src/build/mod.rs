use crate::{project_config::ProjectConfig, runnable::Runnable};
use clap::Args;
use colored::*;
use glob::glob;
use std::{
    env,
    error::Error,
    fs,
    process::{exit, Command, Output},
};

#[derive(Args, Debug)]
pub struct Build {
    #[arg(long)]
    mhz: Option<u8>,
    #[arg(long)]
    release: bool,
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

        Command::new(config.firmware.language.compiler())
            .current_dir(env::current_dir().unwrap())
            // all warnings
            .arg("-Wall")
            // optimization
            .arg(if self.release { "-O3" } else { "-Os" })
            // custom arguments
            .args(arguments)
            .arg(&config.compiler.custom)
            // include header's directory
            .arg(format!("-I{}", config.structure.includes))
            // set arch
            .arg(format!("-mmcu={}", config.firmware.target.to_lowercase()))
            // -o {}/firmware.elf {}/{}.c
            .args(["-o", &format!("{}/firmware.elf", config.structure.builds)])
            .args(get_file_sources(
                &config.structure.sources,
                &config.firmware.language.to_string(),
            ))
            .output()
            .expect("Failed to execute avr-gcc/avr-g++ command")
    }

    fn obj_copy(&self, config: &ProjectConfig) -> Output {
        Command::new("avr-objcopy")
            .current_dir(env::current_dir().unwrap())
            .args(["-j", ".text"])
            .args(["-j", ".data"])
            .args(["-O", "ihex"])
            .args([
                &format!("{}/firmware.elf", config.structure.builds),
                &format!("{}/firmware.hex", config.structure.builds),
            ])
            .output()
            .expect("Failed to execute avr-objcopy command")
    }
}

fn report_error(output: Output) {
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8(output.stderr).unwrap().red());
        exit(1);
    }
}

fn get_file_sources(directory: &str, ext: &str) -> Vec<String> {
    let xs = glob(&format!("{}/**/*.{}", directory, ext)).expect("Failed to read glob pattern");
    xs.map(|file| file.unwrap().display().to_string())
        .collect::<Vec<_>>()
}
