use clap::{Subcommand};

#[derive(Subcommand)]
pub enum PkgmCommand {
    /// Install a package
    Install {
        /// Package to install
        package: String,

        /// Install package globally
        #[arg(short, long)]
        global: bool,
    },

    /// Remove a package
    Remove {
        /// Package to remove
        package: String,

        /// Remove package globally
        #[arg(short, long)]
        global: bool,
    },

    /// Update all packages
    Upgrade {}

}