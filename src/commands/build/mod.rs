use crate::{
    commands::build::compiler_interface::CompilerInterface, project_config::ProjectConfig,
    runnable::Runnable, util::get_list_namefiles,
};
use clap::Args;
use colored::Colorize;
use std::{error::Error, fs};

pub mod compiler_interface;
pub mod objcopy_interface;

#[derive(Args, Debug)]
pub struct Build {
    /// Установить частоту МК
    #[arg(long)]
    mhz: Option<u8>,
    /// Вывести лог всех использованных команд
    #[arg(long)]
    watch: bool,
    /// Указать уровень оптимизаций
    #[arg(long)]
    opt_level: Option<String>,
}

impl Runnable for Build {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let config = ProjectConfig::read_from_file("yug.toml")?;
        self.compile_project(&config)?;
        Ok(())
    }
}

impl Build {
    fn compile_project(&self, config: &ProjectConfig) -> Result<(), String> {
        use compiler_interface::CompilerOption::{Custom, MHz, OptLevel};

        fs::create_dir(&config.structure.builds).unwrap_or(());

        /////////////////////////////////////////////////////////
        // Compilation of project
        let mut compiler_interface = CompilerInterface::new(config);
        compiler_interface.option(MHz(self.mhz.unwrap_or(config.firmware.target.mhz)));
        compiler_interface.option(OptLevel(
            self.opt_level.clone().unwrap_or_else(|| "s".to_string()),
        ));

        // From src/
        compiler_interface.inputs(get_list_namefiles(
            config.structure.sources.as_str(),
            config.firmware.language.to_str(),
        ));

        // From vendor/
        compiler_interface.inputs(get_list_namefiles("vendor", "o"));
        compiler_interface.inputs(
            config
                .externlibs
                .iter()
                .flat_map(|(_, lib)| lib.objs.iter().cloned()),
        );

        // Добавление пользовательских флагов компилятору.
        for arg in config.compiler.args.iter().cloned() {
            compiler_interface.option(Custom(arg));
        }

        // Сборка
        self.log(&compiler_interface.compile()?);
        self.log(&objcopy_interface::objcopy(&config.structure.builds)?);

        Ok(())
    }

    fn log(&self, value: &str) {
        if self.watch {
            println!("$ {}", value.blue());
        }
    }
}
