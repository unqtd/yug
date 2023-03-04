use std::collections::HashMap;

use clap::Args;

use crate::{
    project_config::{Compiler, Firmware, Language, ProjectConfig, Structure, Target, Utils},
    runnable::Runnable,
};

#[derive(Args, Debug)]
pub struct Init {
    /// Название вашего проекта.
    project_name: String,
    /// Модель микроконтроллера.
    #[arg(short, long)]
    target: String,
    /// Установить C++ в качестве языка проекта.
    #[arg(long)]
    cpp: bool,
    /// Расширить базовую структуру проекта.
    #[arg(short, long)]
    dev: bool,
    /// Частота работы микроконтроллера в МГц.
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
        utils::create_sources_directory(&config)?;

        if self.dev {
            // Создание директории с заголовочными файлами
            let includes_path = format!("{}/{}", self.project_name, config.structure.includes);
            utils::create_dir(&includes_path)?;

            self.create_clangd_file()?;
        }

        Ok(())
    }
}

impl Init {
    fn to_config(&self) -> ProjectConfig {
        ProjectConfig {
            firmware: Firmware {
                name: self.project_name.clone(),
                target: Target {
                    model: self.target.clone(),
                    mhz: self.mhz.unwrap_or(1),
                },
                language: (if self.cpp { Language::Cpp } else { Language::C }),
            },
            structure: Structure::default(),
            compiler: Compiler::default(),
            utils: Utils::default(),
            externlibs: HashMap::new(),
        }
    }
}

impl Init {
    fn create_clangd_file(&self) -> Result<(), String> {
        let source = format!(
            r#"
CompileFlags:
  Add:
    - "-I/usr/lib/avr/include"
    - "-I../vendor"
    - "-I../include"
    - "-D__AVR_{target}__"
    - "-DF_CPU={mhz}000000UL"
    "#,
            target = self.target,
            mhz = self.mhz.unwrap_or(1),
        );

        utils::write_str_to_file(&format!("{}/.clangd", self.project_name), source.trim())
    }

    fn create_yug_file(&self, config: &ProjectConfig) -> Result<(), String> {
        utils::write_str_to_file(
            &format!("{}/yug.toml", self.project_name),
            toml::to_string_pretty(config).unwrap().trim_end(),
        )
    }
}

mod utils {
    use std::fs::{self, File};
    use std::io::Write;

    use crate::project_config::ProjectConfig;

    pub fn write_str_to_file(filename: &str, content: &str) -> Result<(), String> {
        let mut file = File::create(filename).map_err(|_| format!("{filename} уже существует!"))?;
        writeln!(&mut file, "{content}")
            .map_err(|_| format!("Ошибка при попытке записи в файл {filename}!"))
    }

    pub fn create_dir(path: &str) -> Result<(), String> {
        fs::create_dir(path).map_err(|_| format!("Директория «{path}» уже существует!"))
    }

    pub fn create_sources_directory(config: &ProjectConfig) -> Result<(), String> {
        let path = format!("{}/{}", config.firmware.name, config.structure.sources);
        create_dir(&path)?;

        write_str_to_file(
            &format!("{path}/main.{ext}", ext = config.firmware.language.to_str()),
            r#"
#include <avr/io.h>

int main(void) {

  while (1) {
  }
}
"#
            .trim(),
        )
    }
}
