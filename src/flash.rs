use std::{env, process::Command};

use clap::Args;

use crate::{project_config::ProjectConfig, runnable::Runnable};

#[derive(Args, Debug)]
pub struct Flash {
    /// Type of programmer
    #[arg(short, long)]
    programmer: String,
    /// Port used programmer
    #[arg(long)]
    port: Option<String>,
}

impl Runnable for Flash {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ProjectConfig::read_from_file("yug.toml")?;
        let path_hex_file = format!("flash:w:{}/firmware.hex:i", &config.structure.builds);

        let mut arguments = vec![
            "-c",
            &self.programmer,
            "-p",
            &config.firmware.target,
            "-U",
            &path_hex_file,
        ];

        if let Some(port) = &self.port {
            arguments.push("-P");
            arguments.push(port)
        }

        Command::new("avrdude")
            .current_dir(env::current_dir().unwrap())
            .args(arguments)
            .arg(format!(
                "flash:w:{}/firmware.hex:i",
                config.structure.builds
            ))
            .spawn()?;

        Ok(())
    }
}
