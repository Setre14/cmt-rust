use std::process::Command;
use std::path::Path;
use std::env;

use crate::util::command_line::CommandLine;

pub fn status(command_line: &CommandLine) -> bool
{
    return status_in_dir(command_line, env::current_dir().unwrap())
}

pub fn status_in_dir<P>(command_line: &CommandLine, current_dir: P) -> bool
where
    P: AsRef<Path> + std::fmt::Debug,
{
    log::debug!("Execute {:#?} in dir {:#?}", command_line, current_dir);
    let status = Command::new(&command_line.command)
        .args(&command_line.args)
        .current_dir(current_dir)
        .status()
        .unwrap();

    log::debug!("Result: {:#?} {:#?}", status.success(), status);

    return status.success();
}