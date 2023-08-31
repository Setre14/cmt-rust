pub mod config;
pub mod util;
pub mod env;
pub mod pkg;
pub mod system_config;

use clap::{Parser, Subcommand};

use config::cli::config_cli::ConfigCli;
use config::cli::config_cli_command::ConfigCliCommand;
use env::cli::env_cli::EnvCli;
use env::cli::env_cli_command::EnvCliCommand;
use config::pojo::local_config::LocalConfig;
use config::pojo::base_config::BaseConfig;
use pkg::cli::pkg_cli::PkgCli;
use pkg::cli::pkg_cli_command::PkgCliCommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Silence all output
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Command>,
}


#[derive(Subcommand)]
enum Command {
    /// Interact with local config
    Config {
        #[command(subcommand)]
        command: ConfigCliCommand,
    },

    /// Manipulate env files
    Env {
        #[command(subcommand)]
        command: EnvCliCommand,

        /// Config to use
        #[arg(short, long, global = true)]
        config: Option<String>,
    },

    /// Interact with package manager
    Pkg {
        #[command(subcommand)]
        command: PkgCliCommand,

        /// Config to use
        #[arg(short, long, global = true)]
        config: Option<String>,
    }
}


fn main() {
    let settings = LocalConfig::get_config(None);

    let cli = Cli::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(cli.quiet)
        .verbosity((cli.verbose + settings.debug_level) as usize)
        .show_module_names(true)
        // .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();
    
    match &cli.command {
        Some(Command::Config { command }) => {
            ConfigCli::handle_command(command);
        },
        Some(Command::Env { command, config }) => {
            EnvCli::handle_command(command, config)
        },
        Some(Command::Pkg { command, config }) => {
            PkgCli::handle_command(command, config)
        },
        None => {} 
    }
}
