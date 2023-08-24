use clap::Subcommand;

use super::system_config_params_add_remove::SystemConfigParamAddRemove;
use super::system_config_params_list::SystemConfigParamList;

#[derive(Subcommand)]
pub enum SystemConfigCliCommand {

    List {
        #[command(flatten)]
        params: SystemConfigParamList,
    },

    Add {
        #[command(flatten)]
        params: SystemConfigParamAddRemove,
    },

    Remove {
        #[command(flatten)]
        params: SystemConfigParamAddRemove,
    }
}
