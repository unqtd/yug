#![allow(dead_code)]

use std::process::Output;

use crate::{
    project_config::{Language, ProjectConfig},
    util::{execute_command, ExecutionMode},
};

#[derive(Clone)]
pub enum CompilerOption<'a> {
    Languge(&'a Language),
    OptLevel(String),
    MHz(u8),
    Custom(String),
}

pub struct CompilerInterface<'a> {
    config: &'a ProjectConfig,

    arguments: Vec<String>,
    sources: Vec<String>,

    output: String,

    languge: Option<&'a Language>,
}

impl<'a> CompilerInterface<'a> {
    pub fn new(config: &'a ProjectConfig) -> Self {
        Self {
            config,
            arguments: Vec::new(),
            sources: Vec::new(),
            output: format!("{}/firmware.elf", config.structure.builds),
            languge: None,
        }
    }

    pub fn sources<I: Iterator<Item = String>>(&mut self, src: I) -> &mut Self {
        self.sources.extend(src);
        self
    }

    pub fn output(&mut self, o: String) -> &mut Self {
        self.output = o;
        self
    }

    pub fn option(&mut self, opt: CompilerOption<'a>) -> &mut Self {
        match opt {
            CompilerOption::Languge(lang) => self.languge = Some(lang),
            option => self.arguments.push(match option {
                CompilerOption::Languge(_) => unreachable!(),
                CompilerOption::OptLevel(lvl) => format!("-O{lvl}"),
                CompilerOption::MHz(mhz) => format!("-DF_CPU={mhz}000000"),
                CompilerOption::Custom(c) => c,
            }),
        };
        self
    }

    pub fn option_from(&mut self, opt: Option<CompilerOption<'a>>) -> &mut Self {
        if let Some(opt) = opt {
            self.option(opt)
        } else {
            self
        }
    }
}

impl<'a> CompilerInterface<'a> {
    pub fn compile(self) -> (Output, String) {
        let headers = format!("-I{}", self.config.structure.includes);
        let mmcu = format!("-mmcu={}", self.config.firmware.target.model.to_lowercase());

        let mut command = vec![
            (self
                .languge
                .unwrap_or(&self.config.firmware.language)
                .compiler()),
            "-Wall",
            "-Ivendor",
            &headers,
            &mmcu,
            "-o",
            &self.output,
        ];
        // Добавлены к команде на вход sources
        command.extend(self.sources.iter().map(String::as_str));
        // Добавлены к команде на вход arguments
        command.extend(self.arguments.iter().map(String::as_str));

        (
            execute_command(&command, ExecutionMode::Output),
            command.join(" "),
        )
    }
}
