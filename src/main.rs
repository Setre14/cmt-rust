pub mod config;
// pub mod env;
pub mod util;
pub mod settings;

use clap::{Parser, Subcommand};

use config::cli::config_cli::{ConfigCli};
use config::cli::config_cli_command::{ConfigCliCommand};
// use env::env_cli::{EnvCli, EnvCliCommand};
use config::config_settings::ConfigSettings;
use settings::base_settings::BaseSettings;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Silence all output
    #[arg(short, long)]
    quiet: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Command>,
}


#[derive(Subcommand)]
enum Command {
    Config {
        #[command(subcommand)]
        command: ConfigCliCommand,
    },

//     Env {
//         #[command(subcommand)]
//         command: EnvCliCommand,
//     }
}


fn main() {
    // let conf = app_config::get_settings();
    let settings = ConfigSettings::get_settings();

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
            ConfigCli::handle_command(command)
        }
        // Some(Command::Env { command }) => {
        //     EnvCli::handle_command(command)
        //     // git_config::init(url, dest, branch, track, *force);
        // }
        None => {} 
    }
}
