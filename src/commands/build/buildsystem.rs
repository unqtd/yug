#![allow(dead_code)]

use std::process::Output;

use crate::{project_config::ProjectConfig, util::get_list_namefiles};

use super::{compiler_interface::CompilerInterface, objcopy_interface::ObjCopyInterface};

#[derive(Clone, Copy)]
pub enum BuildOption<'a> {
    MHz(u8),
    OptLevel(&'a String),
    Custom(&'a str),
}

pub struct BuildSystem<'a> {
    config: &'a ProjectConfig,
    compiler_arguments: Vec<String>,
}

impl<'a> BuildSystem<'a> {
    pub const fn new(config: &'a ProjectConfig) -> Self {
        Self {
            config,
            compiler_arguments: Vec::new(),
        }
    }

    pub fn get_compiler(&self) -> CompilerInterface {
        CompilerInterface::new(
            self.config,
            self.compiler_arguments.iter().map(String::as_str),
        )
    }

    pub fn option(&mut self, opt: BuildOption<'a>) -> &mut Self {
        self.compiler_arguments.push(match opt {
            BuildOption::MHz(mhz) => format!("-DF_CPU={mhz}000000UL"),
            BuildOption::OptLevel(lvl) => format!("-O{lvl}"),
            BuildOption::Custom(custom) => custom.to_string(),
        });

        self
    }

    pub fn option_from(&mut self, opt: Option<BuildOption<'a>>) -> &mut Self {
        opt.map(|opt| self.option(opt));
        self
    }
}

impl<'a> BuildSystem<'a> {
    pub fn compile(&self) -> (Output, String) {
        let mut compiler_interface = self.get_compiler();

        let sources = get_list_namefiles(
            self.config.structure.sources.as_str(),
            self.config.firmware.language.to_str(),
        );

        let headers = get_list_namefiles(self.config.structure.sources.as_str(), "h");

        let externlibs = self
            .config
            .externlibs
            .iter()
            .flat_map(|(_, lib)| lib.objs.iter().map(Clone::clone)); // Лишние копирование!

        let objects = get_list_namefiles("vendor", "o");

        compiler_interface.sources(sources);
        compiler_interface.sources(headers);
        compiler_interface.sources(objects);
        compiler_interface.sources(externlibs);

        compiler_interface.compile()
    }

    pub fn objcopy(&self) -> (Output, String) {
        let objcopy_interface = ObjCopyInterface::new(self.config);
        objcopy_interface.doit()
    }
}
