// SPDX-FileCopyrightText: 2025 Revelation Team
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::SongSummary;

/// Song history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "api", derive(utoipa::ToSchema))]
pub struct SongHistoryEntry {
    pub song:                SongSummary,
    pub transpose_semitones: i16,
    pub viewed_at:           DateTime<Utc>
}
