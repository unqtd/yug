use std::process::Command;

use clap::Args;

use crate::{project_config::ProjectConfig, runnable::Runnable};

#[derive(Args, Debug)]
pub struct Util {
    name: String,
}

impl Runnable for Util {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ProjectConfig::read_from_file("yug.toml")?;

        if let Some(util) = config.utils.utils.get(&self.name) {
            Command::new("sh")
                .arg("-c")
                .arg(util)
                .spawn()?
                .wait_with_output()?;

            Ok(())
        } else {
            Err(format!("Нету такой команды как {}!", self.name).into())
        }
    }
}
