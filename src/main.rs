mod build;
mod flash;
mod init;
mod project_config;
mod runnable;
mod dependence;

use build::Build;
use clap::{Parser, Subcommand};
use colored::*;
use flash::Flash;
use init::Init;
use runnable::Runnable;

#[derive(Parser, Debug)]
#[command(author = "dx3mod")]
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
        // .map_err(|err| eprintln!("{}", format!("Something went wrong:\n{}", err).red()))
    });
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Init a new project
    Init(Init),
    /// Build the project
    Build(Build),
    /// Load firmware to micro-controller
    Flash(Flash),
}

impl Runnable for Commands {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Commands::Init(init) => init.run(),
            Commands::Build(build) => build.run(),
            Commands::Flash(flash) => flash.run(),
        }
    }
}
