use serde::{Serialize, Deserialize};
use strum_macros::Display;
use strum_macros::EnumString;
use strum::{EnumIter};


#[derive(Serialize, Deserialize, Debug, Display, EnumString, EnumIter)]
pub enum PackageManager {
    PACMAN,
    DNF
}

impl Default for PackageManager {
    fn default() -> Self { PackageManager::PACMAN }
}