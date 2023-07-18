use std::process::Command;
use std::ffi::OsStr;
use std::path::Path;
use std::env;

pub fn status<I, S>(program: S, args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    return status_in_dir(program, args, env::current_dir().unwrap())
}

pub fn status_in_dir<I, S, P>(program: S, args: I, current_dir: P) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let status = Command::new(program)
        .args(args)
        .current_dir(current_dir)
        .status()
        .unwrap();

    return status.success();
}