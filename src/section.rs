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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_type_name_ru() {
        assert_eq!(SongSectionType::Verse.name_ru(), "Куплет");
        assert_eq!(SongSectionType::Chorus.name_ru(), "Припев");
        assert_eq!(SongSectionType::Bridge.name_ru(), "Бридж");
        assert_eq!(SongSectionType::PreChorus.name_ru(), "Предприпев");
        assert_eq!(SongSectionType::Intro.name_ru(), "Вступление");
        assert_eq!(SongSectionType::Outro.name_ru(), "Окончание");
        assert_eq!(SongSectionType::Interlude.name_ru(), "Проигрыш");
        assert_eq!(SongSectionType::Tag.name_ru(), "Тег");
        assert_eq!(SongSectionType::Ending.name_ru(), "Кода");
        assert_eq!(SongSectionType::Other.name_ru(), "");
    }
}
