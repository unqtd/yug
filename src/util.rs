use std::process::{Command, Output};

use colored::Colorize;
use glob::glob;

#[derive(Clone, Copy)]
pub enum ExecutionMode {
    Output,
    Spawn,
}

pub fn execute_command(cmds: &[&str], mode: ExecutionMode) -> Output {
    if let (&[prog], args) = cmds.split_at(1) {
        let mut command = Command::new(prog);
        command.args(args.iter().filter(|s| !s.is_empty()));

        match mode {
            ExecutionMode::Output => command.output(),
            ExecutionMode::Spawn => command
                .spawn()
                .and_then(std::process::Child::wait_with_output),
        }
        .unwrap_or_else(|_| panic!("Failed to run '{prog}'"))
    } else {
        todo!()
    }
}

pub fn get_list_namefiles(directory: &str, ext: &str) -> impl Iterator<Item = String> {
    let filenames = glob(&format!("{directory}/**/*.{ext}"))
        .expect("Failed to read glob pattern")
        .map(|filename| filename.unwrap().display().to_string());

    filenames
}

pub fn handle_output(watch: bool, (output, cmd): (Output, String)) -> Result<(), String> {
    if watch {
        print!("$ {}", cmd.blue());
    }

    if output.stderr.is_empty() {
        Ok(())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}
