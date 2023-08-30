use clap::Args;

use crate::config::cli::config_params_update::ConfigParamsUpdate;

#[derive(Args)]
pub struct ConfigParamsInit {
    /// Git clone url
    pub url: String,

    /// Remove folder if destination already exists
    #[arg(short, long)]
    pub force: bool,

    #[command(flatten)]
    pub udpate_params: ConfigParamsUpdate
}
