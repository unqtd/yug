#![allow(dead_code)]

use std::process::Output;

use crate::{
    project_config::{Language, ProjectConfig},
    util::{execute_command, ExecutionMode},
};

#[derive(Clone, Copy)]
pub enum CompilerOption<'a> {
    Languge(&'a Language),
}

pub struct CompilerInterface<'a> {
    config: &'a ProjectConfig,
    arguments: Vec<&'a str>,
    inputs: Vec<&'a str>,
    output: String,
    languge: Option<&'a Language>,
}

impl<'a> CompilerInterface<'a> {
    pub fn new<I>(config: &'a ProjectConfig, arguments: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        Self {
            config,
            arguments: arguments.collect(),
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

    pub fn option(&mut self, opt: CompilerOption<'a>) -> &mut Self {
        match opt {
            CompilerOption::Languge(lang) => self.languge = Some(lang),
        };
        self
    }
}

impl<'a> CompilerInterface<'a> {
    pub fn compile(self) -> (Output, String) {
        let headers = format!("-I{}", self.config.structure.includes);
        let mmcu = format!("-mmcu={}", self.config.firmware.target.to_lowercase());

        let mut command = vec![
            (self
                .languge
                .unwrap_or(&self.config.firmware.language)
                .compiler()),
            "-Wall",
            "-Os",
            "-Ivendor",
            &headers,
            &mmcu,
            "-o",
            &self.output,
        ];
        command.extend(self.inputs);

        (
            execute_command(&command, ExecutionMode::Output),
            command.join(" "),
        )
    }
}
