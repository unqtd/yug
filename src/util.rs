use glob::glob;
use std::process::{Child, Command};

pub fn execute_command(cmds: &[&str]) -> Result<(), String> {
    if let (&[prog], args) = cmds.split_at(1) {
        let mut command = Command::new(prog);
        command.args(args.iter().filter(|s| !s.is_empty()));

        let output = command
            .spawn()
            .and_then(Child::wait_with_output)
            .unwrap_or_else(|_| panic!("Failed to run '{prog}'"));

        let exit_code = output.status.code().expect("Process terminated by signal");

        if exit_code == 0 {
            Ok(())
        } else {
            Err(format!(
                "Ошибка при запуске: «{prog}». Exit code: {exit_code}."
            ))
        }
    } else {
        unreachable!()
    }
}

pub fn get_list_namefiles(directory: &str, ext: &str) -> impl Iterator<Item = String> {
    let filenames = glob(&format!("{directory}/**/*.{ext}"))
        .expect("Failed to read glob pattern")
        .map(|filename| filename.unwrap().display().to_string());

    filenames
}
