// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::SongSummary;

/// User playlist (setlist)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongPlaylist {
    pub id:          Uuid,
    pub user_id:     Uuid,
    pub church_id:   Option<Uuid>,
    pub name:        String,
    pub description: Option<String>,
    pub is_public:   bool,
    pub event_date:  Option<NaiveDate>,
    pub songs_count: i32,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>
}

/// Playlist item with song and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct PlaylistItem {
    pub id:                  Uuid,
    pub song:                SongSummary,
    pub position:            i16,
    pub transpose_semitones: i16,
    pub notes:               Option<String>
}

/// Create playlist request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct CreatePlaylist {
    #[validate(length(min = 1, max = 200))]
    pub name: String,

    pub description: Option<String>,
    pub church_id:   Option<Uuid>,
    pub is_public:   bool,
    pub event_date:  Option<NaiveDate>
}

/// Add song to playlist request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct AddToPlaylist {
    pub song_id:             Uuid,
    pub transpose_semitones: Option<i16>,
    pub notes:               Option<String>
}
