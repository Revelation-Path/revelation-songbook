// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Songbook edition (for historical tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongbookEdition {
    pub id:             Uuid,
    pub songbook_id:    Uuid,
    pub edition_name:   String,
    pub year_published: i16,
    pub songs_count:    i32,
    pub publisher:      Option<String>,
    pub isbn:           Option<String>,
    pub notes:          Option<String>
}
