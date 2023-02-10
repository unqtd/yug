use crate::{
    commands::build::compiler_interface::CompilerInterface,
    project_config::ProjectConfig,
    runnable::Runnable,
    util::{get_list_namefiles, handle_output},
};
use clap::Args;
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
        use compiler_interface::CompilerOption::{MHz, OptLevel};

        fs::create_dir(&config.structure.builds).unwrap_or(());

        /////////////////////////////////////////////////////////
        // Compilation of project
        let mut compiler_interface = CompilerInterface::new(config);
        compiler_interface.option_from(self.mhz.map(MHz));
        compiler_interface.option(
            self.opt_level
                .clone()
                .map_or_else(|| OptLevel("s".to_string()), OptLevel),
        );

        let sources = get_list_namefiles(
            config.structure.sources.as_str(),
            config.firmware.language.to_str(),
        );

        let headers = get_list_namefiles(config.structure.sources.as_str(), "h");

        let externlibs = config
            .externlibs
            .iter()
            .flat_map(|(_, lib)| lib.objs.iter().cloned());

        let objects = get_list_namefiles("vendor", "o");

        compiler_interface.sources(sources);
        compiler_interface.sources(headers);
        compiler_interface.sources(objects);
        compiler_interface.sources(externlibs);

        handle_output(self.watch, compiler_interface.compile())?;

        /////////////////////////////////////////////////////////
        // Proccessing by objcopy
        handle_output(self.watch, objcopy_interface::objcopy(&config.structure.builds))?;

        Ok(println!("Проект был успешно собран."))
    }
}
