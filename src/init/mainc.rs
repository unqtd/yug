use crate::project_config::ProjectConfig;

use super::{write_file, Init};

impl Init {
    pub fn create_main_c(&self, directory: &str, config: &ProjectConfig) {
        write_file(
            format!(
                "{}/{}/main.{ext}",
                &directory,
                config.structure.sources,
                ext = config.firmware.language.to_str()
            ),
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
