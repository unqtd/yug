mod commands;
mod project_config;
mod runnable;
mod util;

use clap::{Parser, Subcommand};
use colored::*;
use commands::{build::Build, deps::Deps, flash::Flash, init::Init};
use runnable::Runnable;

#[derive(Parser, Debug)]
#[command(author = "dx3mod")]
#[command(version = "0.1.8")]
#[command(about = "Ваш верный слуга для игры с микроконтроллерами")]
// #[command(
//     long_about = r"A tool for building projects, working with dependencies, simplified work with the loader."
// )]
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
    /// Создать новый проект
    Init(Init),
    /// Компиляция текущего проекта
    Build(Build),
    /// Прошить МК
    Flash(Flash),
    /// Установка внешних библиотек
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
