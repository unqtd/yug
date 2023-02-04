mod commands;
mod project_config;
mod runnable;
mod util;

use clap::{Parser, Subcommand};
use colored::Colorize;
use commands::{build::Build, flash::Flash, init::Init, util::Util};
use runnable::Runnable;

#[derive(Parser, Debug)]
#[command(author = "dx3mod")]
#[command(version = "0.1.6")]
#[command(about = "Ваш верный слуга для игры с микроконтроллерами")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(Err(err)) = cli.command.map(Runnable::run) {
        eprintln!("{}", err.to_string().red());
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Создать новый проект
    Init(Init),
    /// Компиляция текущего проекта
    Build(Build),
    /// Прошить МК
    Flash(Flash),
    /// Выполнение пользовательской команды
    Util(Util),
}

impl Runnable for Commands {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Init(init) => init.run(),
            Self::Build(build) => build.run(),
            Self::Flash(flash) => flash.run(),
            Self::Util(util) => util.run(),
        }
    }
}
