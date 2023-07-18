mod config;
mod git_config;
mod util;
mod pkgm;

use clap::{Parser, Subcommand};

use config::base;
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

    Config {},

    Env {},

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
        Some(Command::Config {}) => {
            println!("config");
        },
        Some(Command::Env {}) => {
            println!("env");
        },
        Some(Command::Pacman {command}) => {
            Pacman::handle_command(command)
        },
        None => {}
    }
}
