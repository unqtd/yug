use super::{write_file, Init};

impl Init {
    pub fn create_clangd(&self, directory: &str) {
        let source = r#"
CompileFlags:
  Add:
    - "-I/usr/lib/avr/include"
    - "-I../vendor"
    - "-I../include"
    "#
        .trim();

        write_file(format!("{}/.clangd", directory), source)
    }
}
