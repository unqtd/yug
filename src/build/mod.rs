use crate::{project_config::ProjectConfig, runnable::Runnable};
use clap::Args;
use colored::*;
use glob::glob;
use std::{
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
    /// Outputs all used commands
    #[arg(long)]
    watch: bool,
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
        let level_of_optimization = if self.release { "-O3" } else { "-Os" }.to_string();
        format!(
            "{cc} -Wall {optimization} {custom} -I{headers} -mmcu={arch} -o {builds}/firmware.elf {sources}",
            cc = config.firmware.language.compiler(),
            optimization = level_of_optimization,
            custom = format!("{} {}", config.compiler.custom, arguments.join(" ")),
            headers = config.structure.includes,
            arch = config.firmware.target.to_lowercase(),
            builds = config.structure.builds,
            sources = get_file_sources(&config.structure.sources, &config.firmware.language.to_string()).join(" ")
        )
    }

    fn format_avr_objcopy_cmd(&self, config: &ProjectConfig) -> String {
        format!(
            "avr-objcopy -j .text -j .data -O ihex {builds}/firmware.elf {builds}/firmware.hex",
            builds = config.structure.builds
        )
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

fn sh(str: &str, expected: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(str)
        .output()
        .expect(expected)
}
