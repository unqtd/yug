use std::process::{exit, Command, Output};

use colored::Colorize;
use glob::glob;
use itertools::Itertools;

pub fn sh(str: &str, expected: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(str)
        .output()
        .expect(expected)
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
