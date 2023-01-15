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
        let command = self.format_command();

        (
            execute_command(
                &command,
                "Failed to execute avr-objcopy command",
                ExecutionMode::Output,
            ),
            command,
        )
    }

    fn format_command(&self) -> String {
        format!(
            "avr-objcopy -j .text -j .data -O ihex {builds}/firmware.elf {builds}/firmware.hex",
            builds = self.config.structure.builds
        )
    }
}
