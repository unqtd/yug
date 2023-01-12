mod build;
mod deps;
mod flash;
mod init;
mod project_config;
mod runnable;
mod util;

use build::Build;
use clap::{Parser, Subcommand};
use colored::*;
use deps::Deps;
use flash::Flash;
use init::Init;
use runnable::Runnable;

#[derive(Parser, Debug)]
#[command(author = "dx3mod")]
#[command(version = "0.1.1")]
#[command(about = "Your servant for playing with AVR micro-controllers.")]
#[command(
    long_about = r"A tool for building projects, working with dependencies, simplified work with the loader."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    cli.command.map(|command| {
        Runnable::run(command).map_err(|err| eprintln!("{}", format!("{}", err).red()))
    });
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initializes a new project
    Init(Init),
    /// Compiles the current project
    Build(Build),
    /// Download the firmware to the microcontroller
    Flash(Flash),
    /// Install dependencies
    Deps(Deps),
}

impl Runnable for Commands {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Commands::Init(init) => init.run(),
            Commands::Build(build) => build.run(),
            Commands::Flash(flash) => flash.run(),
            Commands::Deps(deps) => deps.run(),
        }
    }
}
