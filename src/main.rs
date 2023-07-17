mod config;
mod git_config;
mod util;

use clap::{Parser, Subcommand};

use config::base;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

    Dnf {},
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Init { url, dest, branch, force }) => {
            git_config::init(url, dest, branch, *force);
        },
        Some(Commands::Config {}) => {
            println!("config");
        },
        Some(Commands::Env {}) => {
            println!("env");
        },
        Some(Commands::Dnf {}) => {
            println!("dnf");
        },

        None => {}
    }
}
