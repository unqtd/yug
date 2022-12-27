use crate::project_config::ProjectConfig;

use super::{write_file, Init};

impl Init {
    pub fn create_main_c(&self, directory: &str, config: &ProjectConfig) {
        write_file(
            format!("{}/{}/main.c", &directory, config.structure.sources),
            format!(
                r#"
{}
#include <avr/io.h>

int main(void) {{

  while (1) {{ 
  }}
}}
            "#,
                if self.include_dir && self.spec {
                    "#include \"spec.h\""
                } else {
                    ""
                }
            )
            .trim(),
        );
    }
}
