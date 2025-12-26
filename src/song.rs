use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::{SongCategory, SongTag};

/// Full song with all details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct Song {
    pub id:              Uuid,
    pub songbook_id:     Option<Uuid>,
    pub songbook_code:   Option<String>,
    pub number:          Option<i32>,
    pub title:           String,
    pub title_alt:       Option<String>,
    pub author_lyrics:   Option<String>,
    pub author_music:    Option<String>,
    pub translator:      Option<String>,
    pub year_written:    Option<i16>,
    pub copyright:       Option<String>,
    pub original_key:    Option<String>,
    pub tempo:           Option<i32>,
    pub time_signature:  Option<String>,
    pub content:         String,
    pub first_line:      String,
    pub categories:      Vec<SongCategory>,
    pub tags:            Vec<SongTag>,
    pub is_favorite:     bool,
    pub user_transpose:  i16,
    pub views_count:     i32,
    pub favorites_count: i32
}

/// Song summary for lists
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongSummary {
    pub id:              Uuid,
    pub songbook_id:     Option<Uuid>,
    pub songbook_code:   Option<String>,
    pub number:          Option<i32>,
    pub title:           String,
    pub author_lyrics:   Option<String>,
    pub first_line:      String,
    pub original_key:    Option<String>,
    pub has_chords:      bool,
    pub categories:      Vec<SongCategory>,
    pub is_favorite:     bool,
    pub views_count:     i32,
    pub favorites_count: i32
}

/// Create song request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct CreateSong {
    pub songbook_id: Option<Uuid>,
    pub number:      Option<i32>,

    #[validate(length(min = 1, max = 300))]
    pub title: String,

    pub title_alt:     Option<String>,
    pub author_lyrics: Option<String>,
    pub author_music:  Option<String>,
    pub translator:    Option<String>,
    pub year_written:  Option<i16>,
    pub copyright:     Option<String>,

    #[validate(length(max = 10))]
    pub original_key: Option<String>,

    #[validate(range(min = 1, max = 300))]
    pub tempo: Option<i32>,

    pub time_signature: Option<String>,

    #[validate(length(min = 1))]
    pub content: String,

    pub categories: Vec<SongCategory>,
    pub tag_ids:    Vec<Uuid>,
    pub source_url: Option<String>
}

/// Update song request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct UpdateSong {
    pub songbook_id: Option<Uuid>,
    pub number:      Option<i32>,

    #[validate(length(min = 1, max = 300))]
    pub title: Option<String>,

    pub title_alt:      Option<String>,
    pub author_lyrics:  Option<String>,
    pub author_music:   Option<String>,
    pub translator:     Option<String>,
    pub year_written:   Option<i16>,
    pub copyright:      Option<String>,
    pub original_key:   Option<String>,
    pub tempo:          Option<i32>,
    pub time_signature: Option<String>,
    pub content:        Option<String>,
    pub categories:     Option<Vec<SongCategory>>,
    pub tag_ids:        Option<Vec<Uuid>>
}
