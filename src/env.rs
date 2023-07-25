// use crate::util::exec;
use crate::config::env;

use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Command {
    Add {
        path: String,
    },
    Apply {
    },
    Remove {
        path: String,
    },
    Sync {
    }
}

pub struct Env {
    
}

impl Env {
    pub fn handle_command(command: &Command) {
        match command {
            Command::Add {path} => {
                Self::add(path);
            },
            Command::Apply {} => {
                Self::apply();
            },
            Command::Remove {path} => {
                Self::remove(path);
            },
            Command::Sync {} => {
                Self::sync();
            }
        }
    }

    pub fn add(path: &String) {
        let mut conf = env::get_conf();

        println!("{}", path);

        conf.add_file(&path);


        println!("{:#?}", conf);
    }

    pub fn apply() {
    }

    pub fn remove(path: &String) {
        let mut conf = env::get_conf();

        println!("{}", path);

        conf.remove_file(&path);


        println!("{:#?}", conf);
    }

    pub fn sync() {
    }
}