use std::process::{exit, Command, Output};

use colored::Colorize;
use glob::glob;
use itertools::Itertools;

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
            ExecutionMode::Spawn => command.spawn().and_then(|x| x.wait_with_output()),
        }
        .unwrap_or_else(|_| panic!("Failed to run '{prog}'"))
    } else {
        todo!()
    }
}

pub fn get_line_of_all_namefiles_in_dir_with_ext(directory: &str, ext: &str) -> String {
    let xs = glob(&format!("{}/**/*.{}", directory, ext)).expect("Failed to read glob pattern");

    Itertools::intersperse(
        xs.map(|file| file.unwrap().display().to_string()),
        " ".to_string(),
    )
    .collect()
}

pub fn report_error(output: Output) {
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8(output.stderr).unwrap().red());
        exit(1);
    }
}

pub fn handle_output(watch: bool, (output, cmd): (Output, String)) {
    if watch {
        println!("{}", cmd.trim())
    }

    report_error(output)
}
