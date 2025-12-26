// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use super::PositionedChord;

/// Parsed song line with chords positioned above text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongLine {
    pub text:   String,
    pub chords: Vec<PositionedChord>
}

/// Song section (verse, chorus, bridge, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongSection {
    pub section_type: SongSectionType,
    pub label:        Option<String>,
    pub lines:        Vec<SongLine>
}

/// Section type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum SongSectionType {
    Verse,
    Chorus,
    Bridge,
    PreChorus,
    Intro,
    Outro,
    Interlude,
    Tag,
    Ending,
    Other
}

impl SongSectionType {
    pub fn name_ru(&self) -> &'static str {
        match self {
            Self::Verse => "Куплет",
            Self::Chorus => "Припев",
            Self::Bridge => "Бридж",
            Self::PreChorus => "Предприпев",
            Self::Intro => "Вступление",
            Self::Outro => "Окончание",
            Self::Interlude => "Проигрыш",
            Self::Tag => "Тег",
            Self::Ending => "Кода",
            Self::Other => ""
        }
    }
}
