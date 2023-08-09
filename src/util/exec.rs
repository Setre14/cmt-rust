use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::env;

use crate::util::command_line::CommandLine;

pub struct Exec{}

impl Exec {    
    pub fn status(command_line: &CommandLine, opt_work_dir: Option<PathBuf>) -> bool
    {
        let work_dir = opt_work_dir.unwrap_or(env::current_dir().unwrap());
        log::debug!("Execute {:?} in dir {:?}", command_line, work_dir);
        let status = Command::new(&command_line.command)
            .args(&command_line.args)
            .current_dir(work_dir)
            // .stdout(Stdio::null())
            // .stderr(Stdio::null())
            .status()
            .unwrap();
    
        log::debug!("Result: {:?} {:?}", status.success(), status);
    
        return status.success();
    }

    pub fn get_hostname() -> String {
        let output = Command::new("hostname")
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap().replace("\n", "");
    
        return stdout;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::util::command_line::CommandLine;
    
    use std::env;
    use pretty_assertions::assert_eq;
    
    #[test]
    fn test_status() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore"].to_vec());

        assert_eq!(Exec::status(&command_line, None), true);
    }    
    
    #[test]
    fn test_status_failed() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore232"].to_vec());

        assert_eq!(Exec::status(&command_line, None), false);
    }
    
    #[test]
    fn test_status_in_dir() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore"].to_vec());

        assert_eq!(Exec::status(&command_line, env::current_dir().ok()), true);
    }

    #[test]
    fn test_status_in_dir_failed() {
        let command_line = CommandLine::create("ls", ["-la", ".gitignore"].to_vec());

        assert_eq!(Exec::status(&command_line, Some(PathBuf::from("/"))), false);
    }
}
