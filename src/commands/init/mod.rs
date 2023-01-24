use clap::Args;

use crate::{
    project_config::{Compiler, Firmware, Language, ProjectConfig, Structure},
    runnable::Runnable,
};

#[derive(Args, Debug)]
pub struct Init {
    /// Название создаваемого проекта
    project_name: String,
    /// Модель МК
    #[arg(short, long)]
    target: String,
    /// Сделать язык C++ языком проекта
    #[arg(long)]
    cpp: bool,
    /// Подготовить полнофункциональную рабочую среду
    #[arg(short, long)]
    dev: bool,
    /// Частота работы МК в МГц
    #[arg(long)]
    mhz: Option<u8>,
}

impl Runnable for Init {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.to_config();

        // Создание директории проекта
        utils::create_dir(&self.project_name)?;

        // Создание файла конфигурации
        self.create_yug_file(&config)?;

        // Создание директории исходников
        self.create_sources_directory(&config)?;

        if self.dev {
            // Создание директории с заголовочными файлами
            let includes_path = format!("{}/{}", self.project_name, config.structure.includes);
            utils::create_dir(&includes_path)?;

            self.create_spec_file(&includes_path)?;
            self.create_clangd_file()?;
        }

        Ok(println!("Готово. 🐪"))
    }
}

impl Init {
    fn to_config(&self) -> ProjectConfig {
        ProjectConfig {
            firmware: Firmware {
                name: self.project_name.to_string(),
                target: self.target.to_string(),
                language: (if self.cpp { Language::Cpp } else { Language::C }),
            },
            structure: Structure::default(),
            compiler: Compiler::default(),
            utils: Default::default(),
        }
    }
}

impl Init {
    fn create_sources_directory(&self, config: &ProjectConfig) -> Result<(), String> {
        let path = format!("{}/{}", config.firmware.name, config.structure.sources);
        utils::create_dir(&path)?;

        utils::write_str_to_file(
            &format!("{path}/main.{ext}", ext = config.firmware.language.to_str()),
            format!(
                r#"
{spec}
#include <avr/io.h>

int main(void) {{

  while (1) {{ 
  }}
}}
            "#,
                spec = if self.dev { "#include \"spec.h\"" } else { "" }
            )
            .trim(),
        )
    }

    fn create_spec_file(&self, path: &str) -> Result<(), String> {
        utils::write_str_to_file(
            &format!("{path}/spec.h"),
            format!(
                r#"
#ifndef __AVR_{avr}__
#define __AVR_{avr}__
#endif
#define F_CPU {}000000UL
                       "#,
                self.mhz.unwrap_or(1),
                avr = self.target,
            )
            .trim(),
        )
    }

    fn create_clangd_file(&self) -> Result<(), String> {
        let source = r#"
CompileFlags:
  Add:
    - "-I/usr/lib/avr/include"
    - "-I../vendor"
    - "-I../include"
    "#
        .trim();

        utils::write_str_to_file(&format!("{}/.clangd", self.project_name), source)
    }

    fn create_yug_file(&self, config: &ProjectConfig) -> Result<(), String> {
        utils::write_str_to_file(
            &format!("{}/yug.toml", self.project_name),
            toml::to_string(config).unwrap().trim_end(),
        )
    }
}

mod utils {
    use std::fs::{self, File};
    use std::io::Write;

    pub fn write_str_to_file(filename: &str, content: &str) -> Result<(), String> {
        let mut file = File::create(filename).map_err(|_| format!("{filename} уже существует!"))?;
        writeln!(&mut file, "{content}")
            .map_err(|_| format!("Ошибка при попытке записи в файл {}!", filename))
    }

    pub fn create_dir(path: &str) -> Result<(), String> {
        fs::create_dir(path).map_err(|_| format!("Директория {path} уже существует!"))
    }
}
