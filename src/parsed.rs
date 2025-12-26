use serde::{Deserialize, Serialize};

use super::SongSection;

/// Fully parsed song structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct ParsedSong {
    pub title:          Option<String>,
    pub subtitle:       Option<String>,
    pub artist:         Option<String>,
    pub composer:       Option<String>,
    pub key:            Option<String>,
    pub tempo:          Option<i32>,
    pub time_signature: Option<String>,
    pub capo:           Option<i32>,
    pub sections:       Vec<SongSection>
}
