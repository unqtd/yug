use std::process::Output;

use crate::util::{execute_command, ExecutionMode};

pub fn objcopy(builds: &str) -> (Output, String) {
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

    (
        execute_command(&command, ExecutionMode::Output),
        command.join(" "),
    )
}
