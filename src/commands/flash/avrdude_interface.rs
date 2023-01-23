use std::process::Output;

use crate::{
    project_config::ProjectConfig,
    util::{execute_command, ExecutionMode},
};

pub enum AvrDudeOption<'a> {
    BitClock(u8),
    BitRate(u8),
    Port(&'a String),
    Target(&'a str),
    Programer(&'a str),
}

pub struct AvrDudeInterface<'a> {
    config: &'a ProjectConfig,
    arguments: Vec<String>,
}

impl<'a> AvrDudeInterface<'a> {
    pub fn new(config: &'a ProjectConfig) -> Self {
        Self {
            config,
            arguments: Vec::new(),
        }
    }

    pub fn option(&mut self, opt: AvrDudeOption<'a>) -> &mut Self {
        // dry...
        self.arguments.push(match opt {
            AvrDudeOption::BitRate(bitrate) => format!("-b{bitrate}"),
            AvrDudeOption::BitClock(bitclock) => format!("-B{bitclock}"),
            AvrDudeOption::Port(port) => format!("-P{port}"),
            AvrDudeOption::Target(target) => format!("-p {target}"),
            AvrDudeOption::Programer(programmer) => format!("-c{programmer}"),
        });
        self
    }

    pub fn option_from(&mut self, opt: Option<AvrDudeOption<'a>>) -> &mut Self {
        match opt {
            Some(x) => self.option(x),
            None => self,
        }
    }
}

impl<'a> AvrDudeInterface<'a> {
    pub fn load(self) -> (Output, String) {
        let uflash = format!("-Uflash:w:{}/firmware.hex:i", self.config.structure.builds);

        let mut command = vec!["avrdude", uflash.as_str()];
        command.extend(self.arguments.iter().map(|x| x.as_str()));

        (
            execute_command(&command, ExecutionMode::Spawn),
            command.join(" "),
        )
    }
}
