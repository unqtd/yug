use std::process::Command;

use super::Init;

impl Init {
    pub fn git_init_repo(&self, directory: &str) {
        if self.git {
            Command::new("git")
                .current_dir(directory)
                .arg("init")
                .output()
                .expect("Failed to git init");
        }
    }
}
