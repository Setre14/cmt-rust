mod config;
mod git_config;
mod util;
mod pkgm;
mod env;

use clap::{Parser, Subcommand};

use config::base;
use config::app;
// use env;
use env::Env;
use pkgm::dnf;
use pkgm::dnf::Dnf;
use pkgm::pacman;
use pkgm::pacman::Pacman;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Silence all output
    #[arg(short, long)]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Init app config and clone git config
    Init {
        /// Git clone url
        url: String,

        /// Destination for git clone, default: ~/.config/cmt-rust
        #[arg(short, long)]
        dest: Option<String>,

        /// Branch to checkout otherwise default branch is used
        #[arg(short, long)]
        branch: Option<String>,

        /// Branch to checkout otherwise default branch is used
        #[arg(short, long)]
        track: Option<String>,

        /// Remove folder if destination already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Interact with git config
    Config {
        #[command(subcommand)]
        command: git_config::Command,
    },

    /// Add, remove and sync files and folders
    Env {
        #[command(subcommand)]
        command: env::Command,
    },

    /// Install, remove and update dnf packages
    Dnf {
        #[command(subcommand)]
        command: dnf::Command,
    },

    /// Install, remove and update pacman packages
    Pacman {
        #[command(subcommand)]
        command: pacman::Command,
    },
}

fn main() {
    let conf = app::get_conf();

    let cli = Cli::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(cli.quiet)
        .verbosity((cli.verbose + conf.debug_level) as usize)
        .show_module_names(true)
        // .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();
    
    match &cli.command {
        Some(Command::Init { url, dest, branch, track, force }) => {
            git_config::init(url, dest, branch, track, *force);
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
