extern crate clap;
extern crate dirs;
extern crate serde;

mod config;
mod git_config;
mod util;
mod pkgm;
mod env;

use clap::{Parser, Subcommand};

use config::base;
// use env;
use env::Env;
use pkgm::dnf;
use pkgm::dnf::Dnf;
use pkgm::pacman;
use pkgm::pacman::Pacman;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Init {
        url: String,

        #[arg(short, long)]
        dest: Option<String>,

        #[arg(short, long)]
        branch: Option<String>,

        #[arg(short, long)]
        force: bool,

    },

    Config {
        #[command(subcommand)]
        command: git_config::Command,
    },

    Env {
        #[command(subcommand)]
        command: env::Command,
    },

    Dnf {
        #[command(subcommand)]
        command: dnf::Command,
    },

    Pacman {
        #[command(subcommand)]
        command: pacman::Command,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Command::Init { url, dest, branch, force }) => {
            git_config::init(url, dest, branch, *force);
        },
        Some(Command::Config {command}) => {
            git_config::handle_command(command)
        },
        Some(Command::Env {command}) => {
            Env::handle_command(command)
        },
        Some(Command::Pacman {command}) => {
            Pacman::handle_command(command)
        },
        Some(Command::Dnf {command}) => {
            Dnf::handle_command(command)
        },
        None => {}
    }
}
