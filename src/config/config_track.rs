use strum_macros::EnumIter;
use strum_macros::Display;
use strum_macros::EnumString;


#[derive(Debug, Clone, Display, EnumString, EnumIter)]
pub enum ConfigTrack {
    GLOBAL,
    SYSTEM
}

#[allow(dead_code)]
pub fn bool_to_track(global: &bool) -> ConfigTrack {
    return match global {
        true => ConfigTrack::GLOBAL,
        false => ConfigTrack::SYSTEM,
    };
}
