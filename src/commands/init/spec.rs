use super::{write_file, Init};

impl Init {
    pub fn create_spec_filel(&self, directory: &str) {
        write_file(
            format!("{}/spec.h", directory),
            format!(
                r#"
#ifndef __AVR_{avr}__
#define __AVR_{avr}__
#endif

#define F_CPU {}000000UL
                       "#,
                self.mhz.unwrap_or(1).to_string(),
                avr = self.target,
            )
            .trim(),
        )
    }
}
