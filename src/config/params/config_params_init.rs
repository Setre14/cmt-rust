use clap::{Args};

use crate::config::params::config_params_update::ConfigParamsUpdate;

#[derive(Args)]
pub struct ConfigParamsInit {
    /// Git clone url
    pub url: String,

    /// Destination for git clone, default: ~/.config/cmt-rust
    #[arg(short, long)]
    pub dest: Option<String>,

    // /// Branch to checkout otherwise default branch is used
    // #[arg(short, long)]
    // pub branch: Option<String>,

    // /// Branch to checkout otherwise default branch is used
    // #[arg(short, long)]
    // pub track: Option<String>,

    /// Remove folder if destination already exists
    #[arg(short, long)]
    pub force: bool,

    #[command(flatten)]
    pub udpate_params: ConfigParamsUpdate
}
