use crate::project_config::ProjectConfig;

use super::{write_file, Init};

impl Init {
    pub fn create_clangd(&self, directory: &str, config: &ProjectConfig) {
        let mut includes = vec![];

        if !self.not_clangd && self.include_dir {
            includes.push(format!("../{}", &config.structure.includes));
        }

        if !self.not_clangd {
            write_file(
                format!("{}/.clangd", &directory),
                format!(
                    r#"
CompileFlags:
  Add:
    - "-I/usr/lib/avr/include"
{}
                "#,
                    includes
                        .iter()
                        .map(|incl| format!("    - \"-I{}\"", incl))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
                .trim(),
            )
        }
    }
}
