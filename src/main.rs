pub mod config;
pub mod util;
pub mod env;
pub mod pkg;

use clap::{Parser, Subcommand};

use config::params::config_params_init::ConfigParamsInit;
use config::params::config_params_update::ConfigParamsUpdate;
use config::config::Config;
use env::cli::env_cli::EnvCli;
use env::cli::env_cli_command::EnvCliCommand;
use config::pojo::local_config::LocalConfig;
use config::pojo::base_config::BaseConfig;
use pkg::cli::pkg_cli::PkgCli;
use pkg::cli::pkg_cli_command::PkgCliCommand;
use pkg::dnf::cli::dnf_cli::DnfCli;
use pkg::dnf::cli::dnf_cli_command::DnfCliCommand;

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

    /// Open config in editor
    Open {
        /// Open git config
        #[arg(short = 'g', long)]
        open_git_config: bool,
    },

    /// Manipulate env files
    Env {
        #[command(subcommand)]
        command: EnvCliCommand,
    },

    /// Interact with dnf package manager
    Pkg {
        #[command(subcommand)]
        command: PkgCliCommand,
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
        Some(Command::Init { params }) => {
            Config::init(params);
        },
        Some(Command::Update { params }) => {
            Config::update(params);
        },
        Some(Command::Open { open_git_config }) => {
            Config::open_in_editor(&settings.editor, open_git_config)
        },
        Some(Command::Env { command }) => {
            EnvCli::handle_command(command)
        },
        Some(Command::Pkg { command }) => {
            PkgCli::handle_command(command)
        },
        None => {} 
    }
}
