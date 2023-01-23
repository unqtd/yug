mod commands;
mod project_config;
mod runnable;
mod util;

use clap::{Parser, Subcommand};
use colored::*;
use commands::{build::Build, flash::Flash, init::Init};
use runnable::Runnable;

#[derive(Parser, Debug)]
#[command(author = "dx3mod")]
#[command(version = "0.1.4")]
#[command(about = "Ваш верный слуга для игры с микроконтроллерами")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        println!(
            r#"
      |\      _,,,---,,_
ZZZzz /,`.-'`'    -.  ;-;;,_
     |,4-  ) )-,_. ,\ (  `'-'
    '---''(_/--'  `-'\_)
                 "#
        )
    }

    cli.command
        .into_iter()
        .for_each(|command| match command.run() {
            Ok(()) => {}
            Err(err) => eprintln!("{}", err.to_string().red()),
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
