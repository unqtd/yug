use std::process::{Command, Output};

use crate::project_config::ProjectConfig;

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
            AvrDudeOption::BitRate(bitrate) => format!("-b{}", bitrate),
            AvrDudeOption::BitClock(bitclock) => format!("-B{}", bitclock),
            AvrDudeOption::Port(port) => format!("-P{}", port),
            AvrDudeOption::Target(target) => format!("-p {}", target),
            AvrDudeOption::Programer(programmer) => format!("-c{}", programmer),
        });
        self
    }

    pub fn option_from(&mut self, opt: Option<AvrDudeOption<'a>>) -> &mut Self {
        opt.map(|x| self.option(x));
        self
    }
}

impl<'a> AvrDudeInterface<'a> {
    pub fn load(self) -> (Output, String) {
        let command = self.format_command();

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .expect("Failed to avrdude")
            .wait_with_output()
            .unwrap();

        (output, command)
    }

    fn format_command(&self) -> String {
        format!(
            "avrdude {args} -Uflash:w:{builds}/firmware.hex:i",
            args = self.arguments.join(" "),
            builds = self.config.structure.builds
        )
    }
}
