use std::process::Output;

use itertools::Itertools;

use crate::{
    project_config::{Language, ProjectConfig},
    util::sh,
};

pub enum CompilerOptions<'a> {
    Languge(&'a Language),
}

pub struct CompilerInterface<'a> {
    config: &'a ProjectConfig,
    arguments: String,
    inputs: Vec<&'a str>,
    output: String,
    languge: Option<&'a Language>,
}

impl<'a> CompilerInterface<'a> {
    pub fn new<I>(config: &'a ProjectConfig, mut arguments: I) -> Self
    where
        I: Iterator<Item = &'a String>,
    {
        Self {
            config,
            arguments: arguments.join(" "),
            inputs: Vec::new(),
            output: format!("{}/firmware.elf", config.structure.builds),
            languge: None,
        }
    }

    pub fn source(&mut self, src: &'a str) -> &mut Self {
        self.inputs.push(src);
        self
    }

    pub fn output(&mut self, o: String) -> &mut Self {
        self.output = o;
        self
    }

    pub fn option(&mut self, opt: CompilerOptions<'a>) -> &mut Self {
        match opt {
            CompilerOptions::Languge(lang) => self.languge = Some(lang),
        };
        self
    }
}

impl<'a> CompilerInterface<'a> {
    pub fn compile(self) -> (Output, String) {
        let command = self.command_format();

        (
            sh(&command, "Failed to execute avr-gcc/avr-g++ command"),
            command,
        )
    }

    pub fn command_format(&self) -> String {
        format!(
            "{cc} -Wall -Os {customs} -Ivendor -I{headers} -mmcu={arch} -o {builds} {sources}",
            cc = self
                .languge
                .unwrap_or(&self.config.firmware.language)
                .compiler(),
            customs = self.arguments,
            headers = self.config.structure.includes,
            arch = self.config.firmware.target.to_lowercase(),
            builds = self.output,
            sources = self.inputs.join(" ")
        )
    }
}
