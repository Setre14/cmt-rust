use std::process::Command;
use std::ffi::OsStr;
use std::path::Path;
use std::env;

pub fn status<I, S>(program: S, args: I) -> bool
where
    I: IntoIterator<Item = S> + std::fmt::Debug,
    S: AsRef<OsStr> + std::fmt::Debug,
{
    return status_in_dir(program, args, env::current_dir().unwrap())
}

pub fn status_in_dir<I, S, P>(program: S, args: I, current_dir: P) -> bool
where
    I: IntoIterator<Item = S> + std::fmt::Debug,
    S: AsRef<OsStr> + std::fmt::Debug,
    P: AsRef<Path> + std::fmt::Debug,
{
    log::debug!("Execute {:#?} {:#?} in dir {:#?}", program, args, current_dir);
    let status = Command::new(program)
        .args(args)
        .current_dir(current_dir)
        .status()
        .unwrap();

    log::debug!("Result: {:#?} {:#?}", status.success(), status);

    return status.success();
}