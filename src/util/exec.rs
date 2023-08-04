use std::process::{Command};
// use std::process::{Command, Stdio};
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
        // .stdout(Stdio::null())
        // .stderr(Stdio::null())
        .status()
        .unwrap();

    log::debug!("Result: {:#?} {:#?}", status.success(), status);

    return status.success();
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::util::command_line::CommandLine;
    
    use std::env;
    use std::path::Path;
    use pretty_assertions::assert_eq;
    
    #[test]
    fn test_status() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore"].to_vec());

        assert_eq!(status(&command_line), true);
    }    
    
    #[test]
    fn test_status_failed() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore232"].to_vec());

        assert_eq!(status(&command_line), false);
    }
    
    #[test]
    fn test_status_in_dir() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore"].to_vec());

        assert_eq!(status_in_dir(&command_line, env::current_dir().unwrap()), true);
    }

    #[test]
    fn test_status_in_dir_failed() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore"].to_vec());

        assert_eq!(status_in_dir(&command_line, &Path::new("/")), false);
    }
}
