use std::{env, process::Command};

use clap::Args;

use crate::{project_config::ProjectConfig, runnable::Runnable};

#[derive(Args, Debug)]
pub struct Flash {
    /// Programmer type
    #[arg(short, long)]
    programmer: String,
    /// The port on which the programmer hangs
    #[arg(long)]
    port: Option<String>,
    #[arg(long)]
    bitrate: Option<String>,
    #[arg(long)]
    bitclock: Option<String>,
    #[arg(long)]
    preset: Option<String>,
}

impl Runnable for Flash {
    fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ProjectConfig::read_from_file("yug.toml")?;
        let path_hex_file = format!("flash:w:{}/firmware.hex:i", &config.structure.builds);

        // ToDo: rewrite!
        if let Some(preset) = self.preset {
            if preset == "nano-old" {
                self.bitrate = Some(String::from("57600"));
            }
        }

        let mut arguments = vec![
            "-c",
            &self.programmer,
            "-p",
            &config.firmware.target,
            "-U",
            &path_hex_file,
        ];

        // ToDo: refactor!
        if let Some(port) = &self.port {
            arguments.push("-P");
            arguments.push(port)
        }

        // dry
        if let Some(bitrate) = &self.bitrate {
            arguments.push("-b");
            arguments.push(bitrate)
        }

        // dry
        if let Some(bitclock) = &self.bitclock {
            arguments.push("-B");
            arguments.push(bitclock)
        }

        Command::new("avrdude")
            .current_dir(env::current_dir().unwrap())
            .args(arguments)
            .arg(format!(
                "flash:w:{}/firmware.hex:i",
                config.structure.builds
            ))
            .spawn()?
            .wait_with_output()?;

        Ok(())
    }
}
