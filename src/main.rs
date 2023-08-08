pub mod config;
pub mod util;
pub mod env;

use clap::{Parser, Subcommand};

use config::params::config_params_init::ConfigParamsInit;
use config::params::config_params_update::ConfigParamsUpdate;
use config::config::Config;
use env::cli::env_cli::EnvCli;
use env::cli::env_cli_command::EnvCliCommand;
use config::pojo::local_config::LocalConfig;
use config::pojo::base_config::BaseConfig;

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
    /// Init local config and clone git config
    Init {
        #[command(flatten)]
        params: ConfigParamsInit,
    },

    /// Update local config
    Update {
        #[command(flatten)]
        params: ConfigParamsUpdate,
    },

    Env {
        #[command(subcommand)]
        command: EnvCliCommand,
    }
}


fn main() {
    // let conf = app_config::get_config();
    let settings = LocalConfig::get_config();

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
        Some(Command::Init { params }) => {
            Config::init(params);
        },
        Some(Command::Update { params }) => {
            Config::update(params);
        },
        Some(Command::Env { command }) => {
            EnvCli::handle_command(command)
            // git_config::init(url, dest, branch, track, *force);
        }
        None => {} 
    }
}
