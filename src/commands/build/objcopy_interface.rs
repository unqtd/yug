use crate::util::execute_command;

pub fn objcopy(builds: &str) -> Result<String, String> {
    let command = [
        "avr-objcopy",
        "-j",
        ".text",
        "-j",
        ".data",
        "-O",
        "ihex",
        &format!("{builds}/firmware.elf"),
        &format!("{builds}/firmware.hex"),
    ];

    execute_command(&command)?;
    Ok(command.join(" "))
}
