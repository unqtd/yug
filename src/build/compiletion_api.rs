use std::process::Output;

use crate::{project_config::ProjectConfig, util::sh};

pub struct CompilerInterface<'a> {
    config: &'a ProjectConfig,
    mhz: Option<String>,
    customs: Vec<&'a str>,
    opt_level: &'a str,
}

impl<'a> CompilerInterface<'a> {
    pub fn from(config: &'a ProjectConfig) -> Self {
        CompilerInterface {
            config,
            mhz: None,
            customs: Vec::new(),
            opt_level: "s",
        }
    }

    pub fn mhz(&mut self, count: u8) {
        self.mhz = Some(format!("-DF_CPU={}000000UL", count));
    }

    pub fn custom(&mut self, value: &'a str) {
        self.customs.push(value);
    }

    pub fn opt_level(&mut self, level: &'a str) {
        // ToDo: make validation
        self.opt_level = level
    }

    // pub fn compile(self) {
    //     report_error(self.gcc_avr().0);
    //     report_error(self.obj_copy().0)
    // }

    pub fn gcc_avr(&self, sources: &str, builds: &str) -> (Output, String) {
        let avr_gcc_cmd = self.format_avr_gcc_cmd(sources, builds);

        (
            sh(&avr_gcc_cmd, "Failed to execute avr-gcc/avr-g++ command"),
            avr_gcc_cmd,
        )
    }

    fn format_avr_gcc_cmd(&self, sources: &str, builds: &str) -> String {
        format!(
            "{cc} -Wall {optimization} {fcpu} {custom} {customs} -Ivendor -I{headers} -mmcu={arch} -o {builds} {sources}",
            cc = self.config.firmware.language.compiler(),
            optimization = format!("-O{}", self.opt_level),
            fcpu = self.mhz.as_ref().unwrap_or(&"".to_string()),
            // ToDo: customize custom args to compiler
            custom = self.config.compiler.custom,
            customs = self.customs.join(" "),
            headers = self.config.structure.includes,
            arch = self.config.firmware.target.to_lowercase(),
            builds = builds,
            sources = sources
            // sources = get_line_of_all_namefiles_in_dir_with_ext(&self.config.structure.sources, self.config.firmware.language.to_str()),
            // yet_sources = match self.config.firmware.language {
            //     Language::C => String::new(),
            //     Language::Cpp => get_line_of_all_namefiles_in_dir_with_ext(&self.config.structure.sources, "c")
            // }
        )
    }

    pub fn obj_copy(&self) -> (Output, String) {
        let obj_copy_cmd = self.format_avr_objcopy_cmd();

        (
            sh(&obj_copy_cmd, "Failed to execute avr-objcopy command"),
            obj_copy_cmd,
        )
    }

    fn format_avr_objcopy_cmd(&self) -> String {
        format!(
            "avr-objcopy -j .text -j .data -O ihex {builds}/firmware.elf {builds}/firmware.hex",
            builds = self.config.structure.builds
        )
    }
}
