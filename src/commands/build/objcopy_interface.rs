use std::process::Output;

use crate::{
    project_config::ProjectConfig,
    util::{execute_command, ExecutionMode},
};

pub struct ObjCopyInterface<'a> {
    config: &'a ProjectConfig,
}

impl<'a> ObjCopyInterface<'a> {
    pub fn new(config: &'a ProjectConfig) -> Self {
        ObjCopyInterface { config }
    }

    pub fn doit(self) -> (Output, String) {
        let builds = &self.config.structure.builds;
        let command = [
            "avr-objcopy",
            "-j",
            ".text",
            "-j",
            ".data",
            "-O",
            "ihex",
            &format!("{builds}/firmware.elf"),
            &format!("{builds}/firmware.hex"),
        ];

        (
            execute_command(&command, ExecutionMode::Output),
            command.join(" "),
        )
    }
}
