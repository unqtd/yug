mod avrdude_interface;

use clap::Args;

use crate::{project_config::ProjectConfig, runnable::Runnable, util::handle_output};

use self::avrdude_interface::{AvrDudeInterface, AvrDudeOption};

#[derive(Args, Debug)]
pub struct Flash {
    /// Тип программатора
    #[arg(short, long, default_value = "usbasp")]
    programmer: String,
    /// Целевой МК. По-умолчанию берётся из конфигурации
    #[arg(short, long)]
    target: Option<String>,
    /// Порт используемый программатором
    #[arg(long)]
    port: Option<String>,
    #[arg(long)]
    bitrate: Option<u8>,
    #[arg(long)]
    bitclock: Option<u8>,
    /// Вывести лог всех использованных команд
    #[arg(long)]
    watch: bool,
}

impl Runnable for Flash {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        use AvrDudeOption::{BitClock, BitRate, Port, Programer, Target};

        let config = ProjectConfig::read_from_file("yug.toml")?;

        let mut avrdude = AvrDudeInterface::new(&config);
        avrdude
            .option(Programer(&self.programmer))
            .option(Target(
                self.target
                    .as_ref()
                    .unwrap_or(&config.firmware.target.model),
            ))
            .option_from(self.port.as_ref().map(Port))
            .option_from(self.bitrate.map(BitRate))
            .option_from(self.bitclock.map(BitClock));

        handle_output(self.watch, avrdude.load()).map_err(Into::into)
    }
}
